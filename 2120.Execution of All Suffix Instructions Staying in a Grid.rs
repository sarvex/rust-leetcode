impl Solution {
    /// Simulates suffix instruction execution on a grid, counting valid steps.
    ///
    /// # Intuition
    /// For each starting instruction index, simulate movement on the grid
    /// until the robot steps out of bounds. The answer for that suffix is
    /// the number of successfully executed instructions.
    ///
    /// # Approach
    /// 1. For each suffix starting at index `i`, begin at `start_pos`.
    /// 2. Execute instructions sequentially, updating position.
    /// 3. Stop when the position goes out of bounds.
    /// 4. Record the count of executed instructions.
    ///
    /// # Complexity
    /// - Time: O(m²) where m is the instruction string length
    /// - Space: O(m)
    pub fn execute_instructions(n: i32, start_pos: Vec<i32>, s: String) -> Vec<i32> {
        let instructions = s.as_bytes();
        let m = instructions.len();

        (0..m)
            .map(|i| {
                let mut row = start_pos[0];
                let mut col = start_pos[1];
                instructions[i..]
                    .iter()
                    .take_while(|&&b| {
                        match b {
                            b'U' => row -= 1,
                            b'D' => row += 1,
                            b'L' => col -= 1,
                            _ => col += 1,
                        }
                        row >= 0 && row < n && col >= 0 && col < n
                    })
                    .count() as i32
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_example() {
        assert_eq!(
            Solution::execute_instructions(3, vec![0, 1], "RRDDLU".to_string()),
            vec![1, 5, 4, 3, 1, 0]
        );
    }

    #[test]
    fn test_single_instruction() {
        assert_eq!(
            Solution::execute_instructions(2, vec![1, 1], "L".to_string()),
            vec![1]
        );
    }

    #[test]
    fn test_immediate_out_of_bounds() {
        assert_eq!(
            Solution::execute_instructions(1, vec![0, 0], "LRUD".to_string()),
            vec![0, 0, 0, 0]
        );
    }
}
