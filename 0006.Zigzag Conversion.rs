impl Solution {
    /// Row-by-row simulation of zigzag character placement.
    ///
    /// # Intuition
    /// Characters in a zigzag pattern bounce between the first and last rows.
    /// Collecting characters into row buckets and concatenating them yields
    /// the zigzag reading order.
    ///
    /// # Approach
    /// Allocate `num_rows` string buffers. Iterate through characters, appending
    /// each to the current row. Toggle direction when the top or bottom row is
    /// reached. Finally, concatenate all row buffers into the result.
    ///
    /// # Complexity
    /// - Time: O(n) — single pass through the string
    /// - Space: O(n) — row buffers hold all characters
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows = num_rows as usize;
        if num_rows == 1 {
            return s;
        }

        let mut rows = vec![String::new(); num_rows];
        let mut current_row = 0;
        let mut going_down = true;

        for ch in s.chars() {
            rows[current_row].push(ch);
            if current_row == 0 || current_row == num_rows - 1 {
                going_down = !going_down;
            }
            if going_down {
                current_row -= 1;
            } else {
                current_row += 1;
            }
        }

        rows.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_rows() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 3),
            "PAHNAPLSIIGYIR"
        );
    }

    #[test]
    fn four_rows() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 4),
            "PINALSIGYAHRPI"
        );
    }

    #[test]
    fn single_row() {
        assert_eq!(Solution::convert("A".to_string(), 1), "A");
    }

    #[test]
    fn rows_exceed_length() {
        assert_eq!(Solution::convert("AB".to_string(), 3), "AB");
    }
}
