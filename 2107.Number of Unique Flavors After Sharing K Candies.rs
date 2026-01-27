use std::collections::HashMap;

impl Solution {
    /// Maximizes unique candy flavors after sharing k consecutive candies.
    ///
    /// # Intuition
    /// We must give away exactly k consecutive candies and keep the rest.
    /// Using a sliding window of size k, we track which flavors remain
    /// outside the window and maximize their count.
    ///
    /// # Approach
    /// 1. Initialize a frequency map with all candies from index k onward.
    /// 2. Slide the window of size k from left to right.
    /// 3. At each step, add the candy leaving the window (left side) and
    ///    remove the candy entering the window (right side).
    /// 4. Track the maximum number of distinct flavors outside the window.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn share_candies(candies: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let n = candies.len();
        let mut freq: HashMap<i32, i32> = HashMap::with_capacity(n);

        candies[k..].iter().for_each(|&c| {
            *freq.entry(c).or_insert(0) += 1;
        });

        let mut max_unique = freq.len() as i32;

        for i in k..n {
            *freq.entry(candies[i - k]).or_insert(0) += 1;

            let count = freq.get_mut(&candies[i]).unwrap();
            *count -= 1;
            if *count == 0 {
                freq.remove(&candies[i]);
            }

            max_unique = max_unique.max(freq.len() as i32);
        }

        max_unique
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_example() {
        assert_eq!(Solution::share_candies(vec![1, 2, 2, 3, 4, 3], 3), 3);
    }

    #[test]
    fn test_all_same_flavor() {
        assert_eq!(Solution::share_candies(vec![2, 2, 2, 2, 2], 2), 1);
    }

    #[test]
    fn test_share_zero() {
        assert_eq!(Solution::share_candies(vec![1, 2, 3], 0), 3);
    }
}
