use std::fs;
use std::path::Path;

fn read_file(path: &[&str]) -> String {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut full = root.to_path_buf();
    for segment in path {
        full.push(segment);
    }

    fs::read_to_string(&full)
        .unwrap_or_else(|e| panic!("Nao foi possivel ler {}: {}", full.display(), e))
}

#[test]
fn case_tag_use_cases_must_require_manager_role_for_mutations() {
    let register_use_case = read_file(&[
        "src",
        "application",
        "use_cases",
        "case_tag",
        "register_case_tag.rs",
    ]);

    let delete_use_case = read_file(&[
        "src",
        "application",
        "use_cases",
        "case_tag",
        "delete_case_tag.rs",
    ]);

    assert!(
        register_use_case.contains("execute(&self, actor_email: &str, name: String)")
            && register_use_case.contains("ensure_is_manager(actor_email)"),
        "RegisterCaseTagUseCase deve exigir actor_email e validar ensure_is_manager"
    );

    assert!(
        delete_use_case.contains("execute(&self, actor_email: &str, id: &str)")
            && delete_use_case.contains("ensure_is_manager(actor_email)"),
        "DeleteCaseTagUseCase deve exigir actor_email e validar ensure_is_manager"
    );
}

#[test]
fn case_tag_endpoints_must_forward_authenticated_email_to_use_cases() {
    let register_endpoint = read_file(&[
        "src",
        "endpoints",
        "case_tags",
        "register_case_tag.rs",
    ]);

    let delete_endpoint = read_file(&[
        "src",
        "endpoints",
        "case_tags",
        "delete_case_tag.rs",
    ]);

    assert!(
        register_endpoint.contains("Extension(claims): Extension<AuthClaims>")
            && register_endpoint.contains(".execute(claims.email.as_str(), payload.name)"),
        "Endpoint register_case_tag deve extrair AuthClaims e repassar claims.email"
    );

    assert!(
        delete_endpoint.contains("Extension(claims): Extension<AuthClaims>")
            && delete_endpoint.contains(".execute(claims.email.as_str(), &tag_id)"),
        "Endpoint delete_case_tag deve extrair AuthClaims e repassar claims.email"
    );
}
