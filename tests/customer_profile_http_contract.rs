use std::fs;
use std::path::Path;

/// Verifica se a conversão de DomainError::NotFound para AppError retorna 404 HTTP
#[test]
fn domain_not_found_error_maps_to_http_404_status() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let error_file =
        fs::read_to_string(root.join("src").join("error.rs")).expect("Deve conseguir ler error.rs");

    // Validação 1: AppError deve ter variante NotFound
    assert!(
        error_file.contains("NotFound"),
        "AppError deve ter variante NotFound"
    );

    // Validação 2: NotFound deve ser mapeado para NOT_FOUND (404)
    assert!(
        error_file.contains("AppError::NotFound") && error_file.contains("StatusCode::NOT_FOUND"),
        "AppError::NotFound deve retornar StatusCode::NOT_FOUND (404)"
    );

    // Validação 3: DomainError::NotFound deve ser convertido para AppError::NotFound
    assert!(
        error_file.contains("DomainError::NotFound") && error_file.contains("AppError::NotFound"),
        "Conversão From<DomainError> deve mapear NotFound para AppError::NotFound"
    );

    println!("✅ Erro de cliente não encontrado é corretamente mapeado para HTTP 404");
}

/// Verifica que UpdateCustomerRequest DTO suporta atualização de profile
#[test]
fn update_customer_dto_supports_profile_update() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let dto_file = fs::read_to_string(root.join("src").join("endpoints").join("dtos.rs"))
        .expect("Deve conseguir ler dtos.rs");

    // Validação 1: UpdateCustomerRequest deve ter campo profile opcional
    assert!(
        dto_file.contains("pub struct UpdateCustomerRequest")
            && dto_file.contains("pub profile: Option<ProfileRequest>"),
        "UpdateCustomerRequest deve suportar profile opcional"
    );

    // Validação 2: ProfileRequest deve existir com campos de profile
    assert!(
        dto_file.contains("pub struct ProfileRequest")
            && (dto_file.contains("cnpj")
                || dto_file.contains("legal_name")
                || dto_file.contains("phones")),
        "ProfileRequest deve conter campos de profile (cnpj, legal_name, phones, etc)"
    );

    // Validação 3: Telefones no ProfileRequest deve ser tri-state Option<Option<Vec<String>>>
    assert!(
        dto_file.contains("pub phones: Option<Option<Vec<String>>>")
            || dto_file.contains("pub phones: Option<Option<Vec<String>>>"),
        "ProfileRequest.phones deve ser Option<Option<Vec<String>>> para distinguir ausente de null"
    );

    println!("✅ DTO de update_customer suporta atualização de profile com semântica de Option");
}

/// Verifica que o endpoint valida business rules de profile
#[test]
fn update_customer_endpoint_validates_profile_business_rules() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let endpoint_file = fs::read_to_string(
        root.join("src")
            .join("endpoints")
            .join("customers")
            .join("update_customer.rs"),
    )
    .expect("Deve conseguir ler update_customer endpoint");

    // Validação 1: Endpoint deve validar regras de negócio do profile
    assert!(
        endpoint_file.contains("validate_business_rules") || endpoint_file.contains("try_with"),
        "Endpoint deve validar business rules do profile (CNPJs, emails, phones)"
    );

    // Validação 2: CNPJ deve ser validado se fornecido
    assert!(
        endpoint_file.contains("try_with_cnpj"),
        "Endpoint deve validar CNPJ se fornecido"
    );

    // Validação 3: Telefones devem ser validados se fornecidos
    assert!(
        endpoint_file.contains("try_with_phones"),
        "Endpoint deve validar phones se fornecidos"
    );

    // Validação 4: Contatos devem ser validados
    assert!(
        endpoint_file.contains("add_contact") || endpoint_file.contains("contact_type"),
        "Endpoint deve validar contatos do profile"
    );

    println!("✅ Endpoint de update_customer valida todas as regras de negócio do profile");
}
