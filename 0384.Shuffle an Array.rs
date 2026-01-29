use rand::Rng;

/// Shuffles an array using the Fisher-Yates algorithm.
///
/// # Intuition
/// Fisher-Yates shuffle produces a uniform random permutation by swapping
/// each element with a randomly chosen element from the remaining positions.
///
/// # Approach
/// 1. Store the original array for reset.
/// 2. On shuffle, clone the array and perform Fisher-Yates swaps.
/// 3. On reset, return a clone of the original.
///
/// # Complexity
/// - Time: O(n) per shuffle/reset
/// - Space: O(n)
struct Solution {
    nums: Vec<i32>,
}

impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        Self { nums }
    }

    fn reset(&self) -> Vec<i32> {
        self.nums.clone()
    }

    fn shuffle(&self) -> Vec<i32> {
        let mut result = self.nums.clone();
        let n = result.len();
        if n <= 1 {
            return result;
        }
        let mut rng = rand::thread_rng();
        for i in (1..n).rev() {
            let j = rng.gen_range(0..=i);
            result.swap(i, j);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_reset_returns_original() {
        let nums = vec![1, 2, 3];
        let solution = Solution::new(nums.clone());
        assert_eq!(solution.reset(), nums);

        // Reset should always return the same original array
        solution.shuffle();
        assert_eq!(solution.reset(), nums);
    }

    #[test]
    fn test_shuffle_preserves_elements() {
        let nums = vec![1, 2, 3, 4, 5];
        let solution = Solution::new(nums.clone());

        let shuffled = solution.shuffle();
        assert_eq!(shuffled.len(), nums.len());

        let original_set: HashSet<_> = nums.into_iter().collect();
        let shuffled_set: HashSet<_> = shuffled.into_iter().collect();
        assert_eq!(original_set, shuffled_set);
    }

    #[test]
    fn test_shuffle_produces_different_permutations() {
        let nums = vec![1, 2, 3, 4, 5, 6];
        let solution = Solution::new(nums.clone());

        let mut permutations = HashSet::new();

        // Run shuffle multiple times and collect different permutations
        for _ in 0..100 {
            let shuffled = solution.shuffle();
            permutations.insert(shuffled);
        }

        // With 6 elements, we should see multiple different permutations
        assert!(
            permutations.len() > 1,
            "Shuffle should produce different permutations"
        );
    }

    #[test]
    fn test_single_element() {
        let nums = vec![42];
        let solution = Solution::new(nums.clone());

        assert_eq!(solution.shuffle(), vec![42]);
        assert_eq!(solution.reset(), vec![42]);
    }

    #[test]
    fn test_empty_array() {
        let nums: Vec<i32> = vec![];
        let solution = Solution::new(nums.clone());

        assert_eq!(solution.shuffle(), Vec::<i32>::new());
        assert_eq!(solution.reset(), Vec::<i32>::new());
    }

    #[test]
    fn test_two_elements() {
        let nums = vec![1, 2];
        let solution = Solution::new(nums.clone());

        let mut saw_original = false;
        let mut saw_swapped = false;

        // With two elements, we should see both [1,2] and [2,1]
        for _ in 0..50 {
            let shuffled = solution.shuffle();
            if shuffled == vec![1, 2] {
                saw_original = true;
            }
            if shuffled == vec![2, 1] {
                saw_swapped = true;
            }
        }

        assert!(
            saw_original && saw_swapped,
            "Should see both permutations with two elements"
        );
    }
}
