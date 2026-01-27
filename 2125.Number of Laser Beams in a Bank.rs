impl Solution {
    /// Counts total laser beams between adjacent device rows.
    ///
    /// # Intuition
    /// Beams form between every pair of security devices on two consecutive
    /// rows that contain devices. The beam count between two rows is the
    /// product of their device counts.
    ///
    /// # Approach
    /// 1. For each row, count the number of '1' characters (devices).
    /// 2. Skip rows with zero devices.
    /// 3. Multiply the current row's count with the previous non-empty row's
    ///    count and accumulate.
    ///
    /// # Complexity
    /// - Time: O(n * m) where n is row count, m is column count
    /// - Space: O(1)
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        bank.iter()
            .map(|row| row.as_bytes().iter().filter(|&&b| b == b'1').count() as i32)
            .filter(|&count| count > 0)
            .fold((0, 0), |(total, prev), cur| (total + prev * cur, cur))
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_example() {
        let bank = vec![
            "011001".to_string(),
            "000000".to_string(),
            "010100".to_string(),
            "001000".to_string(),
        ];
        assert_eq!(Solution::number_of_beams(bank), 8);
    }

    #[test]
    fn test_no_beams() {
        let bank = vec!["000".to_string(), "111".to_string(), "000".to_string()];
        assert_eq!(Solution::number_of_beams(bank), 0);
    }

    #[test]
    fn test_single_row() {
        let bank = vec!["111".to_string()];
        assert_eq!(Solution::number_of_beams(bank), 0);
    }
}
