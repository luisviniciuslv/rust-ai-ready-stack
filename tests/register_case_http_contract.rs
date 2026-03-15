use std::fs;
use std::path::Path;

fn read_file(path: &[&str]) -> String {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut full = root.to_path_buf();
    for segment in path {
        full.push(segment);
    }

    fs::read_to_string(&full)
        .unwrap_or_else(|e| panic!("Não foi possível ler {}: {}", full.display(), e))
}

#[test]
fn register_case_response_must_support_customer_domain_field() {
    let dtos = read_file(&["src", "endpoints", "dtos.rs"]);

    assert!(
        dtos.contains("pub struct CaseResponse")
            && dtos.contains("pub customer_domain: Option<String>"),
        "CaseResponse deve expor customer_domain como campo opcional"
    );

    assert!(
        dtos.contains("#[serde(skip_serializing_if = \"Option::is_none\")]")
            && dtos.contains("pub customer_domain: Option<String>"),
        "customer_domain deve seguir padrão de campo opcional no payload"
    );
}

#[test]
fn register_case_endpoint_must_fill_customer_domain_in_created_response() {
    let endpoint = read_file(&["src", "endpoints", "cases", "register_case.rs"]);

    assert!(
        endpoint.contains("get_customer.execute(&customer_id)")
            || endpoint.contains("get_customer\n            .execute(&customer_id)"),
        "register_case deve buscar customer para obter domínio"
    );

    assert!(
        endpoint.contains("from_case_with_customer_domain"),
        "register_case deve montar resposta com customerDomain"
    );
}

#[test]
fn api_docs_must_show_customer_domain_in_register_case_response_example() {
    let docs = read_file(&["API_ENDPOINTS.md"]);

    assert!(
        docs.contains("### 16) `POST /customer/{customer_id}/case`")
            && docs.contains("\"customerDomain\": \"exemplo.com\""),
        "Documentação do POST /customer/{{customer_id}}/case deve incluir customerDomain no exemplo"
    );
}
