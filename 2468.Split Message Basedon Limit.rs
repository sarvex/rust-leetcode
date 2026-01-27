impl Solution {
    /// Splits a message into parts with `<a/b>` suffixes respecting the character limit.
    ///
    /// # Intuition
    /// The suffix `<a/b>` has variable length depending on the digit counts of both
    /// the part index and total parts. We find the minimum total parts `b` such that
    /// available character space can accommodate the entire message.
    ///
    /// # Approach
    /// 1. Iterate possible values of b (total parts) from 1 to message length
    /// 2. For each b, calculate total available characters by grouping parts by digit count
    /// 3. Find the minimum b where total available >= message length
    /// 4. Construct result by distributing characters across parts
    ///
    /// # Complexity
    /// - Time: O(n * log n) — iterating candidates with O(log n) digit groups each
    /// - Space: O(n) for storing result parts
    pub fn split_message(message: String, limit: i32) -> Vec<String> {
        let limit = limit as usize;
        let msg_len = message.len();

        if msg_len == 0 {
            return vec![];
        }

        let num_parts = Self::find_min_parts(msg_len, limit);
        if num_parts == 0 {
            return vec![];
        }

        Self::construct_parts(&message, num_parts, limit)
    }

    fn find_min_parts(msg_len: usize, limit: usize) -> usize {
        for b in 1..=msg_len {
            let available = Self::calculate_total_available(b, limit);
            if available >= msg_len as i64 {
                return b;
            }

            let d_b = Self::digit_count(b);
            if 3 + 2 * d_b >= limit {
                break;
            }
        }
        0
    }

    fn calculate_total_available(b: usize, limit: usize) -> i64 {
        let d_b = Self::digit_count(b);
        let mut total = 0i64;
        let mut start = 1usize;
        let mut digit_a = 1usize;

        while start <= b {
            let end_of_range = 10usize.saturating_pow(digit_a as u32).saturating_sub(1);
            let end = end_of_range.min(b);
            let count = end - start + 1;
            let suffix_len = 3 + digit_a + d_b;

            if suffix_len >= limit {
                return i64::MIN;
            }

            total += (count as i64) * ((limit - suffix_len) as i64);
            start = end + 1;
            digit_a += 1;
        }

        total
    }

    #[inline]
    fn digit_count(n: usize) -> usize {
        match n {
            0 => 1,
            _ => {
                let mut count = 0;
                let mut num = n;
                while num > 0 {
                    count += 1;
                    num /= 10;
                }
                count
            }
        }
    }

    fn construct_parts(message: &str, num_parts: usize, limit: usize) -> Vec<String> {
        let mut result = Vec::with_capacity(num_parts);
        let msg_bytes = message.as_bytes();
        let msg_len = msg_bytes.len();
        let d_b = Self::digit_count(num_parts);
        let mut idx = 0;

        for a in 1..=num_parts {
            let d_a = Self::digit_count(a);
            let chars_available = limit - (3 + d_a + d_b);
            let end = (idx + chars_available).min(msg_len);

            let mut part = String::with_capacity(limit);
            part.push_str(
                std::str::from_utf8(&msg_bytes[idx..end]).expect("Message should be valid UTF-8"),
            );
            part.push('<');
            part.push_str(&a.to_string());
            part.push('/');
            part.push_str(&num_parts.to_string());
            part.push('>');

            result.push(part);
            idx = end;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let result =
            Solution::split_message("this is really a very awesome message".to_string(), 9);
        let expected = vec![
            "thi<1/14>",
            "s i<2/14>",
            "s r<3/14>",
            "eal<4/14>",
            "ly <5/14>",
            "a v<6/14>",
            "ery<7/14>",
            " aw<8/14>",
            "eso<9/14>",
            "me<10/14>",
            " m<11/14>",
            "es<12/14>",
            "sa<13/14>",
            "ge<14/14>",
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_example_2() {
        let result = Solution::split_message("short message".to_string(), 15);
        assert_eq!(result, vec!["short mess<1/2>", "age<2/2>"]);
    }

    #[test]
    fn test_impossible_small_limit() {
        let result = Solution::split_message("hello".to_string(), 5);
        assert!(result.is_empty());
    }

    #[test]
    fn test_single_part() {
        let result = Solution::split_message("hi".to_string(), 10);
        assert_eq!(result, vec!["hi<1/1>"]);
    }

    #[test]
    fn test_empty_message() {
        let result = Solution::split_message(String::new(), 10);
        assert!(result.is_empty());
    }

    #[test]
    fn test_exact_fit() {
        let result = Solution::split_message("abcdef".to_string(), 8);
        assert_eq!(result, vec!["abc<1/2>", "def<2/2>"]);
    }

    #[test]
    fn test_result_concatenation() {
        let message = "this is a test message".to_string();
        let result = Solution::split_message(message.clone(), 12);
        let reconstructed: String = result
            .iter()
            .map(|part| &part[..part.rfind('<').unwrap()])
            .collect();
        assert_eq!(reconstructed, message);
    }
}
