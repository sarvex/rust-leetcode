
impl Solution {
    /// Finds the maximum score of a sightseeing pair in one pass.
    ///
    /// # Intuition
    /// The score `values[i] + i + values[j] - j` splits into `(values[i] + i)`
    /// and `(values[j] - j)`. Track the best `values[i] + i` seen so far.
    ///
    /// # Approach
    /// Iterate maintaining the maximum `values[i] + i` from previous indices.
    /// At each position j, update the answer with `max_left + values[j] - j`.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
        let mut best = 0;
        let mut max_left = 0;
        for (j, &v) in values.iter().enumerate() {
            best = best.max(max_left + v - j as i32);
            max_left = max_left.max(v + j as i32);
        }
        best
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::max_score_sightseeing_pair(vec![8, 1, 5, 2, 6]),
            11
        );
    }

    #[test]
    fn test_two_elements() {
        assert_eq!(Solution::max_score_sightseeing_pair(vec![1, 2]), 2);
    }
}
