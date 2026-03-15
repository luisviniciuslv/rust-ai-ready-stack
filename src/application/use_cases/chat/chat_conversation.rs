use crate::domain::entities::{Conversation, ConversationMessage};
use crate::domain::ports::{DocumentRepository, EmbeddingGenerator};
use crate::error::{AppError, AppResult};
use async_openai::types::{ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs};
use async_openai::{config::OpenAIConfig, Client};
use async_trait::async_trait;
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct ChatConversationInput {
    pub id: Option<String>,
    pub content: String,
}

pub struct ChatProcessorResponse {
    pub content: String,
    pub category: String,
}

#[async_trait]
pub trait ChatProcessor: Send + Sync {
    async fn process_message(
        &self,
        user_message: &str,
        conversation_history: &str,
    ) -> AppResult<ChatProcessorResponse>;
}

pub struct ChatConversationResult {
    pub id: String,
    pub content: String,
    pub category: String,
    pub time: String,
}

/// Caso de uso: Processar mensagem de conversa com histórico e RAG
pub struct ChatConversationUseCase {
    chat_processor: Arc<dyn ChatProcessor>,
}

struct OpenAiRagChatProcessor {
    openai_client: Client<OpenAIConfig>,
    document_repository: Arc<dyn DocumentRepository>,
    embedding_generator: Arc<dyn EmbeddingGenerator>,
}

struct MessageAnalysis {
    category: String,
    standalone_query: Option<String>,
    needs_rag: bool,
}

impl OpenAiRagChatProcessor {
    fn new(
        openai_client: Client<OpenAIConfig>,
        document_repository: Arc<dyn DocumentRepository>,
        embedding_generator: Arc<dyn EmbeddingGenerator>,
    ) -> Self {
        Self {
            openai_client,
            document_repository,
            embedding_generator,
        }
    }

    async fn analyze_and_reformulate(
        &self,
        message: &str,
        history: &str,
    ) -> AppResult<MessageAnalysis> {
        println!("=== ANÁLISE DE MENSAGEM ===");
        println!("Mensagem do usuário recebida: {}", message);
        println!("Histórico da conversa:\n{}", history);

        let prompt = format!(
            "Analise a conversa abaixo e a última mensagem do usuário. Sabendo que a tudo em nuvem é revendedora de google workspace e microsoft 365. as dúvidas de suporte geralmente são relacionadas a esses serviços.\n\n\
            Histórico:\n{}\n\
            Última mensagem: {}\n\n\
            Responda em JSON PURO, sem markdown. com os campos:\n\
            - 'categoria': 'Comercial', 'Suporte', 'Renovacao', 'Financeiro' ou 'Desconhecido'. (Deve se levar em consideração o contexto do histórico)\n\
            - 'needs_rag': booleano (true se for uma pergunta técnica/processo, false se for conversa casual, mas toda e qualquer pergunta relacionada ao trabalho deve ser true).\n\
            - 'query': Uma pergunta curta e objetiva em português que resuma o que o usuário quer saber, \
            incluindo o contexto do histórico (ex: se ele disse 'quero isso', a query deve ser 'como funciona o produto X').\n\
            Nunca se refira a algo como se já tivesse sido respondido ou mencionado anteriormente, sempre formule como uma pergunta completa. E como se trata de Rag /\
            Sempre use palavras chave se baseando no conteúdo do histórico e da mensagem do usuário, (ex: se ele disse 'E se não for possivel resolver o problema do cliente?' em uma categoria suporte /\
            , a query deve ser 'O que fazer se o nosso suporte não resolver o problema do cliente'\n\
            SE A MENSAGEM FOR CURTA, COMPLEMENTE COM BASE NO HISTÓRICO (ex: primeiro o usuário perguntou Como crio um 'usuário?' e depois 'no workspace'. A query DEVE SER/\
            Como criar um usuário no Workspace\n\"",
            history, message
        );

        println!("[DEBUG] Prompt de análise gerado para OpenAI:\n{}", prompt);

        let raw_json = self.call_openai_model(&prompt).await?;

        println!("[DEBUG] Resposta bruta da OpenAI:\n{}", raw_json);

        let analysis: serde_json::Value = serde_json::from_str(&raw_json).map_err(to_json_error)?;

        let category = analysis["categoria"]
            .as_str()
            .unwrap_or("Desconhecido")
            .to_string();
        let needs_rag = analysis["needs_rag"].as_bool().unwrap_or(false);
        let standalone_query = analysis["query"].as_str().map(|s| s.to_string());

        println!(
            "Análise concluída - Categoria: {}, Precisa RAG: {}, Query reformulada: {:?}",
            category, needs_rag, standalone_query
        );

        Ok(MessageAnalysis {
            category,
            standalone_query,
            needs_rag,
        })
    }

    async fn retrieve_relevant_context(
        &self,
        user_message: &str,
        category: &str,
    ) -> AppResult<String> {
        println!("=== RECUPERAÇÃO DE CONTEXTO ===");
        println!(
            "Buscando contexto para: '{}' (categoria: {})",
            user_message, category
        );

        let query_embedding = self.generate_query_embedding(user_message).await?;
        println!(
            "[DEBUG] Embedding gerado para a query (dimensionalidade: {})",
            query_embedding.len()
        );

        let documents = self
            .document_repository
            .search(query_embedding, 3, Some(category.to_string()))
            .await
            .map_err(to_vector_db_error)?;

        println!("Total de documentos encontrados: {}", documents.len());
        for (idx, doc) in documents.iter().enumerate() {
            println!(
                "[DEBUG] Documento {} (categoria: {}): {} caracteres recuperados",
                idx + 1,
                doc.category,
                doc.content.len()
            );
        }

        let context = self.format_context_from_documents(documents);
        println!("Contexto formatado: {} caracteres", context.len());
        println!("[DEBUG] Contexto completo:\n{}", context);

        Ok(context)
    }

    async fn generate_response(
        &self,
        conversation_history: &str,
        user_message: &str,
        context: &str,
    ) -> AppResult<String> {
        println!("=== GERAÇÃO DE RESPOSTA ===");

        let prompt = self.build_rag_prompt(conversation_history, user_message, context);

        println!("[DEBUG] Prompt do RAG gerado:\n{}", prompt);
        println!("Tamanho do prompt enviado: {} caracteres", prompt.len());

        let response = self.call_openai_model(&prompt).await?;

        println!("Resposta gerada: {} caracteres", response.len());
        println!("[DEBUG] Resposta completa:\n{}", response);

        Ok(response)
    }

    async fn call_openai_model(&self, prompt: &str) -> AppResult<String> {
        let request = CreateChatCompletionRequestArgs::default()
            .model("gpt-4o-mini")
            .messages([ChatCompletionRequestUserMessageArgs::default()
                .content(prompt)
                .build()
                .map_err(to_openai_error)?
                .into()])
            .build()
            .map_err(to_openai_error)?;

        let response = self
            .openai_client
            .chat()
            .create(request)
            .await
            .map_err(to_openai_error)?;

        Ok(response.choices[0]
            .message
            .content
            .clone()
            .unwrap_or_else(|| "Sem resposta".to_string()))
    }

    async fn generate_query_embedding(&self, text: &str) -> AppResult<Vec<f32>> {
        let mut embeddings = self
            .embedding_generator
            .generate_embeddings_batch(vec![text.to_string()])
            .await
            .map_err(AppError::from)?;

        embeddings.pop().ok_or_else(|| {
            AppError::VectorDbError("Nenhum embedding retornado pela OpenAI".to_string())
        })
    }

    fn build_rag_prompt(
        &self,
        conversation_history: &str,
        user_message: &str,
        context: &str,
    ) -> String {
        format!(
            "Você é o assistente dos funcionários da tudo em nuvem.\n\
            Use o contexto para responder à próxima pergunta do usuário. \
            Caso a pergunta não tenha contexto suficiente, \
            responda com 'Não tenho informações suficientes para responder', juntamente pergunte o que é necessário para entender melhor a pergunta do usuário.\n\
            Busque entender o contexto e a mensagem, responda de forma coerente. \
            Nem sempre é necessário enviar tudo que está no contexto.\n\n\
            Contexto:\n{}\n\n\
            Conversa:\n{}\n\n\
            Pergunta: {}\n\
            Assistente:",
            context, conversation_history, user_message
        )
    }

    fn format_context_from_documents(
        &self,
        documents: Vec<crate::domain::entities::DocumentChunk>,
    ) -> String {
        if documents.is_empty() {
            return "Nenhum contexto encontrado.".to_string();
        }

        documents
            .iter()
            .map(|chunk| chunk.content.clone())
            .collect::<Vec<String>>()
            .join("\n---\n")
    }
}

#[async_trait]
impl ChatProcessor for OpenAiRagChatProcessor {
    async fn process_message(
        &self,
        user_message: &str,
        conversation_history: &str,
    ) -> AppResult<ChatProcessorResponse> {
        println!(">>> INICIANDO PROCESSAMENTO DE MENSAGEM <<<");

        let analysis = self
            .analyze_and_reformulate(user_message, conversation_history)
            .await?;

        if !analysis.needs_rag {
            println!("Modo: RESPOSTA DIRETA (sem RAG)");

            let prompt_direto = format!(
                "Você é um assistente aos funcionários da tudo em nuvem e só é chamado quando não \
                é classificado nenhuma intenção clara do funcionário em suas questões. \
                Você deve responder de forma amigavel, mas sempre istigando o funcionário a fazer perguntas \
                relacionadas aos serviços da empresa Tudo em Nuvem. : {}",
                user_message
            );
            println!("[DEBUG] Prompt direto envindo:\n{}", prompt_direto);

            let direct_response = self.call_openai_model(&prompt_direto).await?;

            println!(
                "Resposta direta gerada: {} caracteres",
                direct_response.len()
            );
            println!("[DEBUG] Resposta: {}", direct_response);

            return Ok(ChatProcessorResponse {
                content: direct_response,
                category: analysis.category,
            });
        }

        println!("Modo: RAG (com busca em base de documentos)");

        let query_to_use = analysis.standalone_query.as_deref().unwrap_or(user_message);
        let context = self
            .retrieve_relevant_context(query_to_use, &analysis.category)
            .await?;
        let response_content = self
            .generate_response(conversation_history, user_message, &context)
            .await?;

        println!(">>> PROCESSAMENTO CONCLUÍDO COM SUCESSO <<<");
        println!("Categoria final: {}", analysis.category);

        Ok(ChatProcessorResponse {
            content: response_content,
            category: analysis.category,
        })
    }
}

impl ChatConversationUseCase {
    pub fn new(chat_processor: Arc<dyn ChatProcessor>) -> Self {
        Self { chat_processor }
    }

    pub fn new_openai(
        openai_client: Client<OpenAIConfig>,
        document_repository: Arc<dyn DocumentRepository>,
        embedding_generator: Arc<dyn EmbeddingGenerator>,
    ) -> Self {
        Self::new(Arc::new(OpenAiRagChatProcessor::new(
            openai_client,
            document_repository,
            embedding_generator,
        )))
    }

    pub async fn execute(
        &self,
        conversations: &RwLock<HashMap<String, Conversation>>,
        req: ChatConversationInput,
    ) -> AppResult<ChatConversationResult> {
        println!(">>> Iniciando execução de ChatConversationUseCase");
        println!(
            "Tipo de requisição: {}",
            if req.id.is_some() {
                "continuação"
            } else {
                "nova conversa"
            }
        );

        let (conversation_id, time_user_message, conversation_history) = {
            let mut store = conversations.write().await;

            let conversation_id = match &req.id {
                Some(id) => {
                    println!("Continuando conversa com ID: {}", id);
                    if let Some(conversation) = store.get_mut(id) {
                        conversation.last_interaction = Utc::now();
                        id.clone()
                    } else {
                        return Err(AppError::NotFound(format!(
                            "Conversa com ID {} não encontrada",
                            id
                        )));
                    }
                }
                None => {
                    let new_id = Uuid::new_v4().to_string();
                    println!("Criando nova conversa com ID: {}", new_id);
                    let new_conv = Conversation {
                        id: new_id.clone(),
                        messages: Vec::new(),
                        last_interaction: Utc::now(),
                    };
                    store.insert(new_id.clone(), new_conv);
                    new_id
                }
            };

            let time_user_message = Utc::now();
            let conversation = store.get_mut(&conversation_id).ok_or_else(|| {
                AppError::InternalError(
                    "Falha ao recuperar conversa após criação/localização".to_string(),
                )
            })?;

            println!(
                "Mensagem do usuário adicionada ao histórico: {}",
                req.content
            );
            conversation.messages.push(ConversationMessage {
                content: req.content.clone(),
                time: time_user_message,
                sender: "User".to_string(),
            });

            let conversation_history = build_conversation_history(&conversation.messages);
            println!(
                "[DEBUG] Histórico atualizado com {} mensagens",
                conversation.messages.len()
            );

            (conversation_id, time_user_message, conversation_history)
        };

        let chat_response = self
            .chat_processor
            .process_message(&req.content, &conversation_history)
            .await?;

        println!("Resposta do processador recebida, adicionando ao histórico");

        {
            let mut store = conversations.write().await;
            let conversation = store.get_mut(&conversation_id).ok_or_else(|| {
                AppError::InternalError(
                    "Falha ao recuperar conversa para persistir resposta".to_string(),
                )
            })?;

            conversation.messages.push(ConversationMessage {
                content: chat_response.content.clone(),
                time: Utc::now(),
                sender: "Assistant".to_string(),
            });

            println!(
                "Resposta persistida. Total de mensagens na conversa: {}",
                conversation.messages.len()
            );
        }

        println!("<<< Execução concluída com sucesso");
        println!(
            "ID da conversa: {}, Categoria: {}, Tempo da resposta: {}",
            conversation_id,
            chat_response.category,
            time_user_message.to_rfc3339()
        );

        Ok(ChatConversationResult {
            id: conversation_id,
            content: chat_response.content,
            category: chat_response.category,
            time: time_user_message.to_rfc3339(),
        })
    }
}

fn build_conversation_history(messages: &[ConversationMessage]) -> String {
    let max_messages = 10;

    messages
        .iter()
        .rev()
        .take(max_messages)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .map(|m| {
            format!(
                "-----\nTime: {}\nRole: {}\nMessage: {}",
                m.time.to_rfc3339(),
                m.sender,
                m.content
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn to_openai_error<E>(_: E) -> AppError
where
    E: std::fmt::Display,
{
    AppError::OpenAiError("Falha ao processar requisição de IA".to_string())
}

fn to_json_error<E>(_: E) -> AppError
where
    E: std::fmt::Display,
{
    AppError::JsonError("Falha ao processar payload JSON".to_string())
}

fn to_vector_db_error<E>(_: E) -> AppError
where
    E: std::fmt::Display,
{
    AppError::VectorDbError("Falha ao consultar base vetorial".to_string())
}
