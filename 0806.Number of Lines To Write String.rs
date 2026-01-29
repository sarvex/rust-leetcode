pub struct Solution;

impl Solution {
    /// Calculates lines and last-line width for writing a string.
    ///
    /// # Intuition
    /// Greedily fit characters on the current line. When adding a character
    /// would exceed 100 pixels, start a new line.
    ///
    /// # Approach
    /// Track current line width. For each character, if appending exceeds
    /// 100, increment lines and reset width. Return `[lines, last_width]`.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
        let (lines, last) = s.bytes().fold((1, 0), |(lines, width), b| {
            let w = widths[(b - b'a') as usize];
            if width + w > 100 {
                (lines + 1, w)
            } else {
                (lines, width + w)
            }
        });
        vec![lines, last]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uniform_widths() {
        let widths = vec![10; 26];
        assert_eq!(
            Solution::number_of_lines(widths, "abcdefghijklmnopqrstuvwxyz".to_string()),
            vec![3, 60]
        );
    }

    #[test]
    fn test_varying_widths() {
        let widths = vec![
            4, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
            10, 10, 10, 10,
        ];
        assert_eq!(
            Solution::number_of_lines(widths, "bbbcccdddaaa".to_string()),
            vec![2, 4]
        );
    }
}
