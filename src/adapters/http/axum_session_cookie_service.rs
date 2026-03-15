use crate::domain::ports::SessionCookieService;

pub struct AxumSessionCookieService {
    cookie_name: String,
    cookie_domain: String,
    cookie_path: String,
}

impl AxumSessionCookieService {
    pub fn new(cookie_domain: String) -> Self {
        Self {
            cookie_name: "token".to_string(),
            cookie_domain,
            cookie_path: "/".to_string(),
        }
    }
}

impl SessionCookieService for AxumSessionCookieService {
    fn build_auth_cookie(&self, token: &str) -> String {
        format!(
            "{}={}; Path={}; Domain={}; HttpOnly; Secure; SameSite=Lax",
            self.cookie_name, token, self.cookie_path, self.cookie_domain
        )
    }

    fn build_clear_auth_cookie(&self) -> String {
        format!(
            "{}=; Max-Age=0; Path={}; Domain={}; HttpOnly; Secure; SameSite=Lax",
            self.cookie_name, self.cookie_path, self.cookie_domain
        )
    }
}
