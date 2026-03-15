pub fn normalize_cnpj(cnpj: &str) -> String {
    cnpj.chars().filter(|c| c.is_ascii_digit()).collect()
}

pub fn is_valid_cnpj(cnpj: &str) -> bool {
    let normalized = normalize_cnpj(cnpj);

    if normalized.len() != 14 {
        return false;
    }

    if normalized
        .chars()
        .all(|c| c == normalized.chars().next().unwrap())
    {
        return false;
    }

    let digits: Vec<u32> = normalized
        .chars()
        .map(|c| c.to_digit(10).unwrap_or(99))
        .collect();

    if digits.iter().any(|d| *d > 9) {
        return false;
    }

    let dv1 = calculate_verifier_digit(&digits[..12], &[5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2]);
    let dv2 = calculate_verifier_digit(&digits[..13], &[6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2]);

    digits[12] == dv1 && digits[13] == dv2
}

fn calculate_verifier_digit(base_digits: &[u32], weights: &[u32]) -> u32 {
    let sum: u32 = base_digits
        .iter()
        .zip(weights.iter())
        .map(|(digit, weight)| digit * weight)
        .sum();

    let remainder = sum % 11;
    if remainder < 2 {
        0
    } else {
        11 - remainder
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_cnpj() {
        assert!(is_valid_cnpj("04.252.011/0001-10"));
        assert!(is_valid_cnpj("11222333000181"));
    }

    #[test]
    fn test_invalid_cnpj() {
        assert!(!is_valid_cnpj(""));
        assert!(!is_valid_cnpj("11.111.111/1111-11"));
        assert!(!is_valid_cnpj("12345678000199"));
        assert!(!is_valid_cnpj("123"));
    }

    #[test]
    fn test_normalize_cnpj() {
        assert_eq!(normalize_cnpj("04.252.011/0001-10"), "04252011000110");
    }
}
