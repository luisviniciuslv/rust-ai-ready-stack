/// Validador de domínio com regras similares ao domínio do e-mail
#[allow(dead_code)]
pub fn is_valid_domain(domain: &str) -> bool {
    if domain.is_empty() || domain.len() > 255 {
        return false;
    }

    if domain.starts_with('.') || domain.ends_with('.') || domain.contains("..") {
        return false;
    }

    let domain_parts: Vec<&str> = domain.split('.').collect();
    if domain_parts.len() < 2 {
        return false;
    }

    for part in &domain_parts {
        if part.is_empty() || part.len() > 63 {
            return false;
        }

        if !part.chars().all(|c| c.is_alphanumeric() || c == '-') {
            return false;
        }

        if part.starts_with('-') || part.ends_with('-') {
            return false;
        }
    }

    if let Some(last_part) = domain_parts.last() {
        if last_part.len() < 2 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_domains() {
        assert!(is_valid_domain("example.com"));
        assert!(is_valid_domain("company.com.br"));
        assert!(is_valid_domain("sub-domain.example.com"));
        assert!(is_valid_domain("subsub-domain.example.com.br"));
        assert!(is_valid_domain("EXAMPLE.COM"));
    }

    #[test]
    fn test_invalid_domains() {
        assert!(!is_valid_domain(""));
        assert!(!is_valid_domain("example.com;"));
        assert!(!is_valid_domain("example"));
        assert!(!is_valid_domain("example..com"));
        assert!(!is_valid_domain(".example.com"));
        assert!(!is_valid_domain("example.com."));
        assert!(!is_valid_domain("-example.com"));
        assert!(!is_valid_domain("example-.com"));
        assert!(!is_valid_domain("exa_mple.com"));
        assert!(!is_valid_domain("example.c"));
        assert!(!is_valid_domain("example@domain.com"));
        assert!(!is_valid_domain("example.com/path"));
        assert!(!is_valid_domain("example.com br"));
    }
}
