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
        let mut rng = rand::thread_rng();
        for i in (1..n).rev() {
            let j = rng.gen_range(0..=i);
            result.swap(i, j);
        }
        result
    }
}
