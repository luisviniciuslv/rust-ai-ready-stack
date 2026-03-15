#![allow(dead_code)]

pub fn normalize_phone(phone: &str) -> String {
    phone.chars().filter(|c| c.is_ascii_digit()).collect()
}

pub fn is_valid_phone(phone: &str) -> bool {
    let mut digits = normalize_phone(phone);

    if digits.starts_with("55") && (digits.len() == 12 || digits.len() == 13) {
        digits = digits[2..].to_string();
    }

    if !(digits.len() == 10 || digits.len() == 11) {
        return false;
    }

    if digits.chars().all(|c| c == digits.chars().next().unwrap()) {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_phones() {
        assert!(is_valid_phone("(11) 99876-5432"));
        assert!(is_valid_phone("1132654321"));
        assert!(is_valid_phone("+55 (11) 99876-5432"));
    }

    #[test]
    fn test_invalid_phones() {
        assert!(!is_valid_phone(""));
        assert!(!is_valid_phone("123"));
        assert!(!is_valid_phone("11111111111"));
        assert!(!is_valid_phone("abcd"));
    }

    #[test]
    fn test_normalize_phone() {
        assert_eq!(normalize_phone("+55 (11) 99876-5432"), "5511998765432");
    }
}
