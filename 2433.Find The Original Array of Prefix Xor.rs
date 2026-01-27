impl Solution {
    /// Recovers the original array from its prefix XOR array.
    ///
    /// # Intuition
    /// Since `pref[i] = arr[0] ^ arr[1] ^ ... ^ arr[i]`, we can recover each
    /// element as `arr[i] = pref[i] ^ pref[i-1]` because XOR is its own inverse.
    ///
    /// # Approach
    /// 1. First element is `pref[0]` directly
    /// 2. Each subsequent element is `pref[i] ^ pref[i-1]`
    /// 3. Use `windows(2)` to process consecutive pairs
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) — for the result array
    pub fn find_array(pref: Vec<i32>) -> Vec<i32> {
        std::iter::once(pref[0])
            .chain(pref.windows(2).map(|w| w[1] ^ w[0]))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::find_array(vec![5, 2, 0, 3, 1]),
            vec![5, 7, 2, 3, 2]
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::find_array(vec![13]), vec![13]);
    }

    #[test]
    fn test_all_zeros() {
        assert_eq!(Solution::find_array(vec![0, 0, 0]), vec![0, 0, 0]);
    }

    #[test]
    fn test_two_elements() {
        assert_eq!(Solution::find_array(vec![1, 3]), vec![1, 2]);
    }
}
