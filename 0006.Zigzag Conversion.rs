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
        if num_rows <= 1 {
            return s;
        }

        let input_len = s.len();
        let num_rows = num_rows as usize;
        if num_rows >= input_len {
            return s;
        }

        let avg_row_capacity = (input_len / num_rows).saturating_add(1);
        let mut rows: Vec<String> = (0..num_rows)
            .map(|_| String::with_capacity(avg_row_capacity))
            .collect();
        let mut current_row = 0usize;
        let mut step: isize = 1;
        let last_row = num_rows - 1;

        for ch in s.chars() {
            rows[current_row].push(ch);
            if current_row == 0 {
                step = 1;
            } else if current_row == last_row {
                step = -1;
            }
            current_row = (current_row as isize + step) as usize;
        }

        let mut result = String::with_capacity(input_len);
        for row in rows {
            result.push_str(&row);
        }
        result
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
