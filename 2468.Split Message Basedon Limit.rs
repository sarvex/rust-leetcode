impl Solution {
    /// Split Message Based on Limit
    ///
    /// # Intuition
    /// The suffix `<a/b>` has variable length depending on the digit counts of both
    /// the part index `a` and total parts `b`. We need to find the minimum `b` such
    /// that the total available character space (after accounting for suffixes)
    /// can accommodate the entire message.
    ///
    /// # Approach
    /// 1. Iterate through possible values of `b` (total parts) from 1 to message length
    /// 2. For each `b`, calculate total available characters by grouping parts by their
    ///    digit count (1-9 have 1 digit, 10-99 have 2 digits, etc.)
    /// 3. Find the minimum `b` where total available >= message length
    /// 4. Construct the result by distributing characters across parts
    ///
    /// # Complexity
    /// - Time: O(n * log(n)) where n is message length - iterating through candidates
    ///   with O(log n) digit groups per candidate
    /// - Space: O(n) for storing the result parts
    pub fn split_message(message: String, limit: i32) -> Vec<String> {
        let limit = limit as usize;
        let msg_len = message.len();

        // Edge case: empty message
        if msg_len == 0 {
            return vec![];
        }

        // Find minimum number of parts
        let num_parts = Self::find_min_parts(msg_len, limit);

        if num_parts == 0 {
            return vec![];
        }

        // Construct the result parts
        Self::construct_parts(&message, num_parts, limit)
    }

    /// Finds the minimum number of parts required to split the message
    /// Returns 0 if impossible
    fn find_min_parts(msg_len: usize, limit: usize) -> usize {
        // Maximum possible parts is message length (1 char per part minimum)
        for b in 1..=msg_len {
            let available = Self::calculate_total_available(b, limit);

            if available >= msg_len as i64 {
                return b;
            }

            // Early termination: if we can't fit even single-digit parts anymore
            // the suffix for part b/b is <b/b> with length 3 + 2*digits(b)
            let d_b = Self::digit_count(b);
            if 3 + 2 * d_b >= limit {
                break;
            }
        }

        0
    }

    /// Calculates total available characters for exactly `b` parts
    /// Returns negative value if impossible
    fn calculate_total_available(b: usize, limit: usize) -> i64 {
        let d_b = Self::digit_count(b);
        let mut total: i64 = 0;
        let mut start = 1_usize;
        let mut digit_a = 1_usize;

        while start <= b {
            // End of current digit range: 9, 99, 999, ...
            let end_of_range = 10_usize.saturating_pow(digit_a as u32).saturating_sub(1);
            let end = end_of_range.min(b);
            let count = end - start + 1;

            // Suffix format: <a/b> has length 3 + digits(a) + digits(b)
            let suffix_len = 3 + digit_a + d_b;

            if suffix_len >= limit {
                // Cannot fit any characters in these parts
                return i64::MIN;
            }

            let chars_per_part = limit - suffix_len;
            total += (count as i64) * (chars_per_part as i64);

            start = end + 1;
            digit_a += 1;
        }

        total
    }

    /// Counts the number of digits in a positive integer
    #[inline]
    fn digit_count(n: usize) -> usize {
        if n == 0 {
            return 1;
        }

        let mut count = 0;
        let mut num = n;

        while num > 0 {
            count += 1;
            num /= 10;
        }

        count
    }

    /// Constructs the result parts with proper suffixes
    fn construct_parts(message: &str, num_parts: usize, limit: usize) -> Vec<String> {
        let mut result = Vec::with_capacity(num_parts);
        let msg_bytes = message.as_bytes();
        let msg_len = msg_bytes.len();
        let d_b = Self::digit_count(num_parts);
        let mut idx = 0;

        for a in 1..=num_parts {
            let d_a = Self::digit_count(a);
            let suffix_len = 3 + d_a + d_b;
            let chars_available = limit - suffix_len;

            let end = (idx + chars_available).min(msg_len);

            // Build the part string efficiently
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
        let message = "this is really a very awesome message".to_string();
        let limit = 9;
        let result = Solution::split_message(message, limit);

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
        let message = "short message".to_string();
        let limit = 15;
        let result = Solution::split_message(message, limit);

        let expected = vec!["short mess<1/2>", "age<2/2>"];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_impossible_small_limit() {
        // Limit too small to even fit suffix <1/1> (5 chars) + 1 char
        let message = "hello".to_string();
        let limit = 5;
        let result = Solution::split_message(message, limit);

        assert!(result.is_empty());
    }

    #[test]
    fn test_single_part() {
        let message = "hi".to_string();
        let limit = 10;
        let result = Solution::split_message(message, limit);

        assert_eq!(result, vec!["hi<1/1>"]);
    }

    #[test]
    fn test_empty_message() {
        let message = "".to_string();
        let limit = 10;
        let result = Solution::split_message(message, limit);

        assert!(result.is_empty());
    }

    #[test]
    fn test_exact_fit() {
        // Message that fits exactly in parts
        let message = "abcdef".to_string();
        let limit = 8;
        // <1/2> is 5 chars, so 3 chars available = "abc"
        // <2/2> is 5 chars, so 3 chars available = "def"
        let result = Solution::split_message(message, limit);

        assert_eq!(result, vec!["abc<1/2>", "def<2/2>"]);
    }

    #[test]
    fn test_digit_transition() {
        // Test case where parts span from single to double digits
        let message = "a]".repeat(20); // 40 chars
        let limit = 10;
        let result = Solution::split_message(message, limit);

        // Verify all parts have valid format
        assert!(!result.is_empty());
        for (i, part) in result.iter().enumerate() {
            assert!(part.len() <= limit);
            assert!(part.contains(&format!("<{}/", i + 1)));
        }
    }

    #[test]
    fn test_result_concatenation() {
        let message = "this is a test message".to_string();
        let limit = 12;
        let result = Solution::split_message(message.clone(), limit);

        // Verify concatenation equals original message
        let reconstructed: String = result
            .iter()
            .map(|part| {
                let suffix_start = part.rfind('<').unwrap();
                &part[..suffix_start]
            })
            .collect();

        assert_eq!(reconstructed, message);
    }
}
