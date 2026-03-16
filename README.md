# Rust AI-Ready Stack

Boilerplate backend em Rust com Axum + Clean Architecture, preparado para:

- autenticação moderna (JWT + Google OAuth opcional)
- modo de login local para desenvolvimento
- ingestão e busca de contexto para RAG (LanceDB + OpenAI)
- infraestrutura MongoDB com TLS opcional e configurável por ambiente

## Arquitetura

- `src/domain`: entidades e portas de domínio (auth, user, documentos, serviços)
- `src/application`: casos de uso de autenticação e RAG
- `src/adapters`: integrações (MongoDB, LanceDB, OpenAI, Google OAuth, JWT, HTTP)
- `src/endpoints`: rotas HTTP expostas
- `src/state.rs` e `src/main.rs`: composição da aplicação

## Pré-requisitos

### Obrigatórios (local e produção)

- Rust toolchain (stable)
- MongoDB acessível (`MONGODB_URI`)
- `JWT_SECRET` configurado no ambiente

### Obrigatórios para recursos de IA

- `OPENAI_API_KEY` para rotas de chat/ingestão que usam embeddings/completion

### Opcionais

- Google OAuth (`GOOGLE_CLIENT_ID` / `GOOGLE_CLIENT_SECRET`)
- Mongo Express (via `docker compose --profile tools`)
- TLS no MongoDB (`MONGODB_TLS_*`)

## Setup rápido (Local)

### 1) Criar ambiente

PowerShell:

```powershell
Copy-Item .env.example .env
```

bash:

```bash
cp .env.example .env
```

### 2) Subir MongoDB (opcional, mas recomendado para dev local)

```bash
docker compose up -d mongodb
```

Mongo Express (opcional):

```bash
docker compose --profile tools up -d
```

### 3) Configurar `protoc` (somente se necessário)

Windows:

```powershell
cargo protoc-setup
```

Linux/macOS:

```bash
cargo protoc-setup
```

> Se quiser detalhes, veja [PROTOC_SETUP.md](PROTOC_SETUP.md).

### 4) Rodar aplicação

```bash
cargo run
```

Servidor padrão: `http://localhost:5555`

## Rotas disponíveis

### Públicas

- `GET /auth` (inicia fluxo Google OAuth)
- `GET /auth/callback` (callback Google)
- `POST /auth/local` (login local, se habilitado)
- `POST /sign-out`
- `POST /ingest` (ingestão de arquivo para RAG)

### Protegidas (JWT em cookie)

- `POST /chat`
- `GET /profile`

## Variáveis de ambiente

Arquivo de referência: [.env.example](.env.example)

### DATABASE

- `MONGODB_URI` **(obrigatória)**
- `MONGODB_DB_NAME` *(opcional, default: `rust_ai_ready_stack_db`)*
- `MONGODB_TLS_ENABLED` *(opcional, default: `false` em dev)*
- `MONGODB_TLS_ALLOW_INVALID_CERTIFICATES` *(opcional, default: `true` quando TLS ativo)*
- `MONGODB_TLS_CA_FILE` *(opcional)*
- `MONGODB_TLS_CERT_KEY_FILE` *(opcional)*
- `PORT` *(opcional, default: `5555`)*

Também existem variáveis de bootstrap para `docker-compose`:

- `MONGODB_ROOT_USERNAME`, `MONGODB_ROOT_PASSWORD`, `MONGODB_PORT`
- `MONGO_EXPRESS_PORT`, `MONGO_EXPRESS_USERNAME`, `MONGO_EXPRESS_PASSWORD`

### AUTH

- `JWT_SECRET` **(obrigatória)**
- `URL_BACKEND` *(opcional, recomendado em produção)*
- `URL_FRONTEND` *(opcional, recomendado em produção)*
- `COOKIE_DOMAIN` *(opcional, default: `localhost`)*
- `GOOGLE_CLIENT_ID` *(opcional; habilita OAuth quando preenchido com secret)*
- `GOOGLE_CLIENT_SECRET` *(opcional; habilita OAuth quando preenchido com client id)*
- `GOOGLE_REQUIRE_VERIFIED_EMAIL` *(opcional, default: `true`)*
- `GOOGLE_ALLOWED_EMAIL_DOMAINS` *(opcional; lista separada por vírgula)*
- `LOCAL_AUTH_ENABLED` *(opcional, default: `true`)*
- `LOCAL_AUTH_EMAIL`, `LOCAL_AUTH_PASSWORD`, `LOCAL_AUTH_NAME`, `LOCAL_AUTH_PICTURE` *(opcionais para login local)*

### EMAIL

- `SMTP_EMAIL_ADDRESS` *(opcional, extensão futura)*
- `SMTP_EMAIL_PASSWORD` *(opcional, extensão futura)*

### AI_CONFIG

- `OPENAI_API_KEY` **(obrigatória para usar chat/ingestão com OpenAI)**

## TLS do MongoDB

No boilerplate atual, TLS é **configurável por ambiente** e **opcional em desenvolvimento**.

### Dev local (recomendado)

```dotenv
MONGODB_TLS_ENABLED="false"
```

### Produção (recomendado)

```dotenv
MONGODB_TLS_ENABLED="true"
MONGODB_TLS_ALLOW_INVALID_CERTIFICATES="false"
MONGODB_TLS_CA_FILE="/path/to/ca.pem"
MONGODB_TLS_CERT_KEY_FILE="/path/to/client.pem"
```

## Produção: checklist mínimo

- Definir `JWT_SECRET` forte e rotacionável
- Definir `URL_BACKEND`, `URL_FRONTEND`, `COOKIE_DOMAIN` corretos
- Usar HTTPS no edge/proxy
- Habilitar TLS no MongoDB (`MONGODB_TLS_ENABLED=true`)
- Definir `MONGODB_TLS_ALLOW_INVALID_CERTIFICATES=false`
- Configurar `GOOGLE_ALLOWED_EMAIL_DOMAINS` (quando OAuth habilitado)
- Desabilitar login local (`LOCAL_AUTH_ENABLED=false`) se não for necessário
- Não versionar `.env` com segredos

## Comandos úteis

- Build/check:

```bash
cargo check
```

- Executar servidor:

```bash
cargo run
```

- Setup de `protoc`:

```bash
cargo protoc-setup
```

- Limpeza de `protoc`:

```bash
cargo protoc-clean
```

## Notas

- `.env.example` é template versionado.
- `.env` deve ser local e conter segredos reais.
- O projeto mantém componentes opcionais para extensão futura (ex.: SMTP, repositório de exemplo MongoDB).
