impl Solution {
    /// Interleave two halves of the array.
    ///
    /// # Intuition
    /// The shuffled result alternates elements from the first and second halves:
    /// `[x1, y1, x2, y2, ...]`. Zipping both halves produces the interleaving.
    ///
    /// # Approach
    /// 1. Split the array at index n
    /// 2. Zip both halves and flatten pairs
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) for the result
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let n = n as usize;
        nums[..n]
            .iter()
            .zip(nums[n..].iter())
            .flat_map(|(x, y)| [*x, *y])
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_shuffle() {
        assert_eq!(
            Solution::shuffle(vec![2, 5, 1, 3, 4, 7], 3),
            vec![2, 3, 5, 4, 1, 7]
        );
    }

    #[test]
    fn four_elements() {
        assert_eq!(
            Solution::shuffle(vec![1, 2, 3, 4, 4, 3, 2, 1], 4),
            vec![1, 4, 2, 3, 3, 2, 4, 1]
        );
    }

    #[test]
    fn two_elements() {
        assert_eq!(Solution::shuffle(vec![1, 1, 2, 2], 2), vec![1, 2, 1, 2]);
    }
}
