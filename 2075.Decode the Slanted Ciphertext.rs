impl Solution {
    /// Decodes the slanted ciphertext by reconstructing diagonals from the row-major matrix.
    ///
    /// # Intuition
    /// The original text was placed along successive diagonals of a `rows × cols` matrix,
    /// then encoded by reading row-by-row. Reversing the process means reading each
    /// diagonal `(0, c), (1, c+1), (2, c+2), …` from the flat row-major layout.
    ///
    /// # Approach
    /// 1. Compute `cols = len / rows`.
    /// 2. For each starting column `c` in `0..cols`, walk the diagonal collecting
    ///    characters from the encoded string using index `r * cols + (c + r)`.
    /// 3. Trim trailing spaces (the original text has none).
    ///
    /// # Complexity
    /// - Time: O(n) where n = encoded_text.len()
    /// - Space: O(n)
    pub fn decode_ciphertext(encoded_text: String, rows: i32) -> String {
        let rows = rows as usize;
        let n = encoded_text.len();
        if n == 0 || rows <= 1 {
            return encoded_text;
        }
        let cols = n / rows;
        let bytes = encoded_text.as_bytes();
        let mut result: String = (0..cols)
            .flat_map(|sc| {
                (0..rows)
                    .map(move |r| (r, sc + r))
                    .take_while(move |&(_, c)| c < cols)
                    .map(|(r, c)| bytes[r * cols + c] as char)
            })
            .collect();
        result.truncate(result.trim_end().len());
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_cipher() {
        assert_eq!(
            Solution::decode_ciphertext("ch   ie   pr".to_string(), 3),
            "cipher"
        );
    }

    #[test]
    fn test_example_leetcode() {
        assert_eq!(
            Solution::decode_ciphertext("iveo    eed   l te   olc".to_string(), 4),
            "i love leetcode"
        );
    }

    #[test]
    fn test_single_row() {
        assert_eq!(
            Solution::decode_ciphertext("coding".to_string(), 1),
            "coding"
        );
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::decode_ciphertext(String::new(), 5), "");
    }

    #[test]
    fn test_single_char() {
        assert_eq!(Solution::decode_ciphertext("a".to_string(), 1), "a");
    }
}
