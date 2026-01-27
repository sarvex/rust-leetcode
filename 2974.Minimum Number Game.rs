impl Solution {
    /// Simulates the minimum number game by sorting and swapping adjacent pairs.
    ///
    /// # Intuition
    /// Both Alice and Bob pick the minimum each turn. After sorting, Alice picks
    /// element at even index, Bob at odd index. Bob's pick goes first in the result,
    /// effectively swapping each adjacent pair.
    ///
    /// # Approach
    /// 1. Sort the array using `sort_unstable`.
    /// 2. Iterate over consecutive pairs using `chunks_exact(2)`.
    /// 3. Push each pair in reversed order (Bob's value first, then Alice's).
    ///
    /// # Complexity
    /// - Time: O(n log n) for sorting
    /// - Space: O(n) for the result vector
    pub fn number_game(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable();
        let mut ans = Vec::with_capacity(nums.len());
        nums.chunks_exact(2).for_each(|pair| {
            ans.push(pair[1]);
            ans.push(pair[0]);
        });
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_game() {
        assert_eq!(Solution::number_game(vec![5, 4, 2, 3]), vec![3, 2, 5, 4]);
    }

    #[test]
    fn test_two_elements() {
        assert_eq!(Solution::number_game(vec![2, 5]), vec![5, 2]);
    }
}
