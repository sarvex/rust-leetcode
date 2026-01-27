impl Solution {
    /// Counts how many times "balloon" can be formed from the text.
    ///
    /// # Intuition
    /// Count occurrences of b, a, l, o, n. 'l' and 'o' need double counts.
    /// The answer is the minimum ratio.
    ///
    /// # Approach
    /// Count frequencies of the five relevant characters. Divide 'l' and 'o'
    /// counts by 2. Return the minimum.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut freq = [0i32; 26];
        for b in text.bytes() {
            freq[(b - b'a') as usize] += 1;
        }
        let counts = [
            freq[(b'b' - b'a') as usize],
            freq[(b'a' - b'a') as usize],
            freq[(b'l' - b'a') as usize] / 2,
            freq[(b'o' - b'a') as usize] / 2,
            freq[(b'n' - b'a') as usize],
        ];
        *counts.iter().min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::max_number_of_balloons("nlaebolko".to_string()), 1);
    }

    #[test]
    fn test_two() {
        assert_eq!(
            Solution::max_number_of_balloons("loonbalxballpoon".to_string()),
            2
        );
    }

    #[test]
    fn test_none() {
        assert_eq!(Solution::max_number_of_balloons("leetcode".to_string()), 0);
    }
}
