/// Validador de emails usando regex simples
/// Valida o formato básico de um email
pub fn is_valid_email(email: &str) -> bool {
    // Validação simples: deve conter @ e pelo menos um caractere em cada lado
    // Padrão: algo@algo.algo
    if email.is_empty() {
        return false;
    }

    // Dividir em local e domínio
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return false;
    }

    let local_part = parts[0];
    let domain_part = parts[1];

    // Validar parte local (antes do @)
    if local_part.is_empty() || local_part.len() > 64 {
        return false;
    }

    // Validar caracteres especiais na parte local
    if !local_part
        .chars()
        .all(|c| c.is_alphanumeric() || ".-_+".contains(c))
    {
        return false;
    }

    // Validar que não começa ou termina com ponto
    if local_part.starts_with('.') || local_part.ends_with('.') {
        return false;
    }

    // Validar que não tem pontos consecutivos
    if local_part.contains("..") {
        return false;
    }

    // Validar domínio (depois do @)
    if domain_part.is_empty() || domain_part.len() > 255 {
        return false;
    }

    // Validar que tem pelo menos um ponto no domínio
    if !domain_part.contains('.') {
        return false;
    }

    // Validar caracteres do domínio
    let domain_parts: Vec<&str> = domain_part.split('.').collect();
    for part in &domain_parts {
        if part.is_empty() {
            return false;
        }
        // Cada parte do domínio pode ter letras, números e hífens
        if !part.chars().all(|c| c.is_alphanumeric() || c == '-') {
            return false;
        }
        // Não pode começar ou terminar com hífen
        if part.starts_with('-') || part.ends_with('-') {
            return false;
        }
    }

    // A TLD (última parte do domínio) deve ter pelo menos 2 caracteres
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
    fn test_valid_emails() {
        assert!(is_valid_email("user@example.com"));
        assert!(is_valid_email("test.user@example.com"));
        assert!(is_valid_email("user+tag@example.co.uk"));
        assert!(is_valid_email("a@example.com"));
        assert!(is_valid_email("user_name@example-domain.com"));
    }

    #[test]
    fn test_invalid_emails() {
        assert!(!is_valid_email(""));
        assert!(!is_valid_email("invalid"));
        assert!(!is_valid_email("@example.com"));
        assert!(!is_valid_email("user@"));
        assert!(!is_valid_email("user@.com"));
        assert!(!is_valid_email("user..name@example.com"));
        assert!(!is_valid_email(".user@example.com"));
        assert!(!is_valid_email("user.@example.com"));
        assert!(!is_valid_email("user@domain"));
        assert!(!is_valid_email("user@-example.com"));
        assert!(!is_valid_email("user@example-.com"));
        assert!(!is_valid_email("user@example.c"));
    }
}
