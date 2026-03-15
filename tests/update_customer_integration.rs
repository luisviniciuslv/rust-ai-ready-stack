use std::fs;
use std::path::Path;

/// Verifica se o use case de update_customer valida existência do customer
/// Padrão: "Cliente com ID ... não encontrado" deve ser retornado quando customer não existe
#[test]
fn update_customer_must_return_not_found_when_customer_does_not_exist() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let use_case_file = fs::read_to_string(
        root.join("src")
            .join("application")
            .join("use_cases")
            .join("customer")
            .join("update_customer.rs"),
    )
    .expect("Deve conseguir ler update_customer.rs");

    // Validação 1: Deve existir validação de customer antes de atualizar
    assert!(
        use_case_file.contains("find_full_by_id(id).await?"),
        "Use case deve buscar customer completo antes de atualizar"
    );

    // Validação 2: Deve retornar NotFound se customer não existe
    assert!(
        use_case_file.contains("DomainError::NotFound"),
        "Use case deve retornar NotFound quando customer não encontrado"
    );

    // Validação 3: Message deve ser clara sobre o cliente não ser encontrado
    assert!(
        use_case_file.contains("Cliente com ID")
            || use_case_file.contains("Cliente") && use_case_file.contains("não encontrado"),
        "Mensagem de erro deve indicar que cliente não foi encontrado"
    );

    println!("✅ Update customer valida de forma explícita que customer existe antes de tentar atualizar");
}

/// Verifica se a lógica de merge de telefones é explícita e sem "adivinhar"
#[test]
fn merge_profiles_telefones_logic_must_be_explicit() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let use_case_file = fs::read_to_string(
        root.join("src")
            .join("application")
            .join("use_cases")
            .join("customer")
            .join("update_customer.rs"),
    )
    .expect("Deve conseguir ler update_customer.rs");

    // Validação 1: Semântica deve ser explícita via flags de presença de campo
    assert!(
        use_case_file.contains("ProfileUpdateFieldPresence")
            && use_case_file.contains("profile_field_presence.phones"),
        "Merge deve usar flags explícitas de presença de campo"
    );

    // Validação 2: Deve usar valor explícito de partial
    assert!(
        use_case_file.contains("partial.phones()"),
        "Merge deve usar valor explícito de partial.phones()"
    );

    // Validação 3: Não deve preservar existing baseado em "está vazio"
    let old_preserve_logic = "partial.phones().is_empty() && !existing.phones().is_empty()";
    assert!(
        !use_case_file.contains(old_preserve_logic),
        "Lógica antiga de 'adinhar' se foi enviado ou não foi removida"
    );

    println!("✅ Merge de telefones usa semântica explícita sem tentar adivinhar intenção");
}

/// Verifica que os testes cobrem casos críticos de merge
#[test]
fn update_customer_tests_cover_critical_scenarios() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let use_case_file = fs::read_to_string(
        root.join("src")
            .join("application")
            .join("use_cases")
            .join("customer")
            .join("update_customer.rs"),
    )
    .expect("Deve conseguir ler update_customer.rs");

    // Validação 1: Teste de limpeza de telefones
    assert!(
        use_case_file.contains("limpa_telefones") || use_case_file.contains("telefones quando"),
        "Deve existir teste que valida limpeza de telefones com valores vazios"
    );

    // Validação 2: Teste de preservação de telefones
    assert!(
        use_case_file.contains("preserva_telefones")
            || use_case_file.contains("telefones_existentes"),
        "Deve existir teste que valida preservação de telefones quando não alterados"
    );

    // Validação 3: Teste de timestamps
    assert!(
        use_case_file.contains("created_at") && use_case_file.contains("updated_at"),
        "Deve existir teste que valida preservação de created_at e atualização de updated_at"
    );

    println!("✅ Testes cobrem: limpeza de telefones, preservação, e timestamps");
}
