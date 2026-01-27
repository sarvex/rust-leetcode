impl Solution {
    /// Validates whether a string is a valid IPv4, IPv6, or neither.
    ///
    /// # Intuition
    /// IPv4 has four dot-separated decimal groups (0–255, no leading zeros).
    /// IPv6 has eight colon-separated hex groups (1–4 hex digits each).
    ///
    /// # Approach
    /// 1. Check for dots to attempt IPv4 validation.
    /// 2. Check for colons to attempt IPv6 validation.
    /// 3. Return "Neither" if both fail.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) for split allocation
    pub fn valid_ip_address(query_ip: String) -> String {
        if Self::is_ipv4(&query_ip) {
            "IPv4".to_string()
        } else if Self::is_ipv6(&query_ip) {
            "IPv6".to_string()
        } else {
            "Neither".to_string()
        }
    }

    fn is_ipv4(s: &str) -> bool {
        if s.ends_with('.') || s.starts_with('.') {
            return false;
        }
        let parts: Vec<&str> = s.split('.').collect();
        parts.len() == 4
            && parts.iter().all(|p| {
                !p.is_empty()
                    && p.len() <= 3
                    && !(p.len() > 1 && p.starts_with('0'))
                    && p.chars().all(|c| c.is_ascii_digit())
                    && p.parse::<u16>().map_or(false, |v| v <= 255)
            })
    }

    fn is_ipv6(s: &str) -> bool {
        if s.ends_with(':') || s.starts_with(':') {
            return false;
        }
        let parts: Vec<&str> = s.split(':').collect();
        parts.len() == 8
            && parts
                .iter()
                .all(|p| !p.is_empty() && p.len() <= 4 && p.chars().all(|c| c.is_ascii_hexdigit()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipv4() {
        assert_eq!(
            Solution::valid_ip_address("172.16.254.1".to_string()),
            "IPv4"
        );
    }

    #[test]
    fn test_ipv6() {
        assert_eq!(
            Solution::valid_ip_address("2001:0db8:85a3:0000:0000:8a2e:0370:7334".to_string()),
            "IPv6"
        );
    }

    #[test]
    fn test_neither() {
        assert_eq!(
            Solution::valid_ip_address("256.256.256.256".to_string()),
            "Neither"
        );
    }

    #[test]
    fn test_leading_zero() {
        assert_eq!(
            Solution::valid_ip_address("01.01.01.01".to_string()),
            "Neither"
        );
    }
}
