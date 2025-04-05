/// Reverses only the vowels in a string, keeping all other characters in their original positions.
///
/// # Intuition
/// Use direct byte manipulation with unsafe code for performance, working with mutable string bytes.
///
/// # Approach
/// 1. Convert the string to mutable bytes for in-place manipulation
/// 2. Use iterators to find vowels from both ends of the string
/// 3. Swap vowels when found from both ends
/// 4. Continue until no more vowel pairs can be found
///
/// # Complexity
/// - Time complexity: O(n) where n is the length of the string
/// - Space complexity: O(1) as we modify the string in-place
///
/// # Parameters
/// - `s`: The input string to process
///
/// # Returns
/// A string with vowels reversed but all other characters in their original positions
impl Solution {
    pub fn reverse_vowels(mut s: String) -> String {
        if s.len() <= 1 {
            return s;
        }

        // From the description, `s` is guaranteed to be ASCII, so this is safe
        let bytes = unsafe { s.as_bytes_mut() };

        let mut left = 0;
        let mut right = bytes.len() - 1;

        // Local lambda function to check if a byte is a consonant (not a vowel)
        let is_consonant = |c: u8| -> bool {
            !matches!(c.to_ascii_lowercase(), b'a' | b'e' | b'i' | b'o' | b'u')
        };

        while left < right {
            while left < right && is_consonant(bytes[left]) {
                left += 1;
            }

            while left < right && is_consonant(bytes[right]) {
                right -= 1;
            }

            if left < right {
                bytes.swap(left, right);
                left += 1;
                right -= 1;
            }
        }

        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        input: String,
        expected: String,
    }

    #[test]
    fn test_reverse_vowels() {
        let test_cases = [
            TestCase {
                input: "hello".to_string(),
                expected: "holle".to_string(),
            },
            TestCase {
                input: "leetcode".to_string(),
                expected: "leotcede".to_string(),
            },
            TestCase {
                input: "aA".to_string(),
                expected: "Aa".to_string(),
            },
            TestCase {
                input: "".to_string(),
                expected: "".to_string(),
            },
            TestCase {
                input: "a".to_string(),
                expected: "a".to_string(),
            },
            TestCase {
                input: "bcdfghjkl".to_string(), // No vowels
                expected: "bcdfghjkl".to_string(),
            },
            TestCase {
                input: "aeiou".to_string(), // All vowels
                expected: "uoiea".to_string(),
            },
            TestCase {
                input: "AEIOU".to_string(), // All uppercase vowels
                expected: "UOIEA".to_string(),
            },
            TestCase {
                input: "aEiOu".to_string(), // Mixed case vowels
                expected: "uOiEa".to_string(),
            },
        ];

        for case in test_cases {
            assert_eq!(
                Solution::reverse_vowels(case.input.clone()),
                case.expected,
                "Failed for input: {}",
                case.input
            );
        }
    }
}
