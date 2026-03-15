use std::fs;
use std::path::Path;

fn load_main_rs() -> String {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    fs::read_to_string(root.join("src").join("main.rs"))
        .unwrap_or_else(|e| panic!("Não foi possível ler src/main.rs: {}", e))
}

fn section_between<'a>(content: &'a str, start: &str, end: &str) -> &'a str {
    let start_idx = content
        .find(start)
        .unwrap_or_else(|| panic!("Trecho inicial não encontrado: {}", start));

    let after_start = &content[start_idx..];
    let end_relative_idx = after_start
        .find(end)
        .unwrap_or_else(|| panic!("Trecho final não encontrado: {}", end));

    &after_start[..end_relative_idx]
}

fn normalize_whitespace(content: &str) -> String {
    content.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[test]
fn protected_routes_must_use_require_jwt_cookie_layer() {
    let main_rs = load_main_rs();
    let protected = section_between(
        &main_rs,
        "let protected_routes = Router::new()",
        "let public_routes = Router::new()",
    );

    assert!(
        protected.contains(".layer(axum_middleware::from_fn_with_state("),
        "Bloco protected_routes precisa declarar layer com from_fn_with_state"
    );
    assert!(
        protected.contains("require_jwt_cookie"),
        "Bloco protected_routes precisa usar require_jwt_cookie"
    );
}

#[test]
fn auth_required_routes_must_be_declared_in_protected_routes() {
    let main_rs = load_main_rs();
    let protected = section_between(
        &main_rs,
        "let protected_routes = Router::new()",
        "let public_routes = Router::new()",
    );
    let protected_normalized = normalize_whitespace(protected);

    let expected_protected_route_parts = [
        ("/chat", "post(chat_conversation)"),
        ("/profile", "get(get_user_profile)"),
        ("/customer/{customer_id}", "get(get_customer)"),
        ("/customer/{customer_id}", "delete(delete_customer)"),
        ("/customer/{customer_id}", "patch(update_customer)"),
        ("/case/{case_id}/ticket", "get(list_tickets_by_case_id)"),
        ("/case/{case_id}/ticket", "post(register_ticket_in_case)"),
        ("/case/{case_id}", "get(get_case)"),
        (
            "/customer/{customer_id}/case",
            "get(list_cases_by_customer_id)",
        ),
        ("/case/{case_id}/close", "patch(close_case)"),
        ("/case/{case_id}/reopen", "patch(reopen_case)"),
        ("/case/{case_id}", "delete(delete_case)"),
        ("/ticket/{ticket_id}", "patch(update_ticket_content)"),
        ("/ticket/{ticket_id}", "delete(delete_ticket)"),
        ("/department", "post(register_user_in_department)"),
        ("/department", "get(list_users_by_department)"),
        ("/department/{user_id}", "delete(remove_user_department)"),
        ("/departments", "get(list_departments)"),
        ("/customer", "get(list_domains)"),
        ("/customer", "post(register_customer)"),
        ("/customer/{customer_id}/case", "post(register_case)"),
        ("/case-tag", "post(register_case_tag)"),
        ("/case-tag", "get(list_case_tags_by_name)"),
        ("/case-tag/{tag_id}", "get(get_case_tag)"),
        ("/case-tag/{tag_id}", "delete(delete_case_tag)"),
    ];

    for (path, handler_call) in expected_protected_route_parts {
        assert!(
            protected_normalized.contains(&format!("\"{}\"", path))
                && protected_normalized.contains(handler_call),
            "Rota protegida ausente do bloco protected_routes: path='{}', handler='{}'",
            path,
            handler_call
        );
    }
}

#[test]
fn auth_required_routes_must_not_be_declared_in_public_routes() {
    let main_rs = load_main_rs();
    let public = section_between(
        &main_rs,
        "let public_routes = Router::new()",
        "let app = Router::new()",
    );

    let auth_required_paths = [
        "/chat",
        "/profile",
        "/customer/{customer_id}",
        "/case/{case_id}/ticket",
        "/case/{case_id}",
        "/customer/{customer_id}/case",
        "/case/{case_id}/close",
        "/case/{case_id}/reopen",
        "/ticket/{ticket_id}",
        "/department",
        "/department/{user_id}",
        "/departments",
        "/customer",
        "/case-tag",
        "/case-tag/{tag_id}",
    ];

    for path in auth_required_paths {
        let route_declaration = format!("route(\"{}\"", path);
        assert!(
            !public.contains(&route_declaration),
            "Rota que exige autenticação não pode estar em public_routes: {}",
            path
        );
    }
}
