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
fn user_domain_must_define_notification_structure() {
    let user_entity = read_file(&["src", "domain", "entities", "user.rs"]);

    assert!(
        user_entity.contains("pub struct Notification")
            && user_entity.contains("id: String")
            && user_entity.contains("case_id: String")
            && user_entity.contains("message: String")
            && user_entity.contains("concluded: bool"),
        "User domain deve declarar Notification com id, case_id, message e concluded"
    );

    assert!(
        user_entity.contains("notifications: Vec<Notification>"),
        "User deve manter lista de notifications no domínio"
    );
}

#[test]
fn mongo_user_document_must_be_backward_compatible_without_notifications_field() {
    let models = read_file(&["src", "adapters", "mongodb", "models.rs"]);

    assert!(
        models.contains("pub notifications: Option<Vec<NotificationDocument>>")
            && models.contains("#[serde(default, skip_serializing_if = \"Option::is_none\")]")
            && models.contains("if let Some(notifications) = self.notifications"),
        "UserDocument deve aceitar usuários legados sem notifications e mapear quando existir"
    );
}

#[test]
fn mongo_notification_document_must_include_unique_id() {
    let models = read_file(&["src", "adapters", "mongodb", "models.rs"]);

    assert!(
        models.contains("pub struct NotificationDocument")
            && models.contains("pub id: String")
            && models.contains("#[serde(default = \"default_notification_id\")]")
            && models.contains("Notification::from_persisted"),
        "NotificationDocument deve persistir id único e suportar documentos legados sem id"
    );
}

#[test]
fn register_ticket_use_case_must_create_notifications_for_pending_users() {
    let use_case = read_file(&[
        "src",
        "application",
        "use_cases",
        "ticket",
        "register_ticket_in_case.rs",
    ]);

    assert!(
        use_case.contains("Notification::new")
            && use_case.contains("case_id.to_string()")
            && use_case.contains("notification_message")
            && use_case.contains("false"),
        "Use case deve criar Notification com case_id, message=content e concluded=false"
    );

    assert!(
        use_case.contains("add_notification_to_users(&[pending_user_id], notification)"),
        "Use case deve enviar notification para o pending_user_id informado"
    );
}

#[test]
fn mongo_user_repository_must_push_notifications_to_existing_or_missing_field() {
    let user_repo = read_file(&["src", "adapters", "mongodb", "user_repository.rs"]);

    assert!(
        user_repo.contains("update_many")
            && user_repo.contains("\"notifications\": notification_bson")
            && user_repo.contains("\"_id\": { \"$in\": object_ids }"),
        "Repositório Mongo deve fazer push de notification para múltiplos usuários por id"
    );
}

#[test]
fn mongo_user_repository_must_update_notification_concluded_by_notification_id() {
    let user_repo = read_file(&["src", "adapters", "mongodb", "user_repository.rs"]);

    assert!(
        user_repo.contains("update_notification_concluded")
            && user_repo.contains("\"notifications.id\": notification_id")
            && user_repo.contains("\"notifications.$.concluded\": concluded"),
        "Repositório Mongo deve atualizar concluded usando notification_id"
    );
}

#[test]
fn notification_patch_endpoint_must_accept_json_with_concluded_bool() {
    let main_rs = read_file(&["src", "main.rs"]);
    let endpoint = read_file(&[
        "src",
        "endpoints",
        "users",
        "update_notification_concluded.rs",
    ]);

    assert!(
        main_rs.contains("/notification/{notification_id}")
            && main_rs.contains("patch(update_notification_concluded)"),
        "main.rs deve expor rota PATCH para atualizar concluded de notificação"
    );

    assert!(
        endpoint.contains("Json(payload): Json<UpdateNotificationConcludedRequest>")
            && endpoint.contains("payload.concluded"),
        "Endpoint PATCH de notificação deve receber body JSON com concluded(bool) e persistir o valor recebido"
    );
}
