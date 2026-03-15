use crate::domain::ports::SessionCookieService;
use std::sync::Arc;

pub struct SignOutUseCase {
    session_cookie_service: Arc<dyn SessionCookieService>,
}

impl SignOutUseCase {
    pub fn new(session_cookie_service: Arc<dyn SessionCookieService>) -> Self {
        Self {
            session_cookie_service,
        }
    }

    pub fn execute(&self) -> String {
        self.session_cookie_service.build_clear_auth_cookie()
    }
}
