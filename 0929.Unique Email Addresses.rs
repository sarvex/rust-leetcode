use std::collections::HashSet;

impl Solution {
    /// Counts unique email addresses after applying local name rules.
    ///
    /// # Intuition
    /// Normalize each email by removing dots and ignoring everything after
    /// '+' in the local part, then count distinct addresses.
    ///
    /// # Approach
    /// Split each email at '@'. Process the local part by filtering dots
    /// and truncating at '+'. Collect normalized emails in a `HashSet`.
    ///
    /// # Complexity
    /// - Time: O(n * L) where n is email count and L is max email length
    /// - Space: O(n * L) for the set
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        emails
            .iter()
            .map(|email| {
                let mut parts = email.splitn(2, '@');
                let local = parts.next().unwrap();
                let domain = parts.next().unwrap();
                let normalized: String = local
                    .bytes()
                    .take_while(|&b| b != b'+')
                    .filter(|&b| b != b'.')
                    .map(|b| b as char)
                    .collect();
                format!("{normalized}@{domain}")
            })
            .collect::<HashSet<_>>()
            .len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let emails = vec![
            "test.email+alex@leetcode.com",
            "test.e.mail+bob.cathy@leetcode.com",
            "testemail+david@lee.tcode.com",
        ]
        .into_iter()
        .map(String::from)
        .collect();
        assert_eq!(Solution::num_unique_emails(emails), 2);
    }

    #[test]
    fn test_all_different() {
        let emails = vec!["a@leetcode.com", "b@leetcode.com", "c@leetcode.com"]
            .into_iter()
            .map(String::from)
            .collect();
        assert_eq!(Solution::num_unique_emails(emails), 3);
    }
}
