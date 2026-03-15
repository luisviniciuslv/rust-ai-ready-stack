pub trait SessionCookieService: Send + Sync {
    fn build_auth_cookie(&self, token: &str) -> String;
    fn build_clear_auth_cookie(&self) -> String;
}
