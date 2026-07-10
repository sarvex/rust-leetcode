impl Solution {
    /// Greedy sort: find maximum value achievable after rearranging and decreasing.
    ///
    /// # Intuition
    /// After sorting, the optimal strategy is to make the array as close to [1, 2, 3, ...]
    /// as possible. Each element can only contribute `min(arr[i], prev + 1)` where `prev`
    /// is the previous element's final value. The answer is the last element's value.
    ///
    /// # Approach
    /// 1. Sort the array in ascending order.
    /// 2. Force the first element to 1 (only decreases are allowed, and it must be 1).
    /// 3. For each subsequent element, set it to `min(arr[i], prev + 1)`.
    ///    - Can't exceed `arr[i]` (can only decrease).
    ///    - Can't exceed `prev + 1` (adjacent difference constraint).
    /// 4. Return the last element — it's the maximum achievable value.
    ///
    /// # Complexity
    /// - Time: O(n log n) for sorting
    /// - Space: O(1) extra (sort in place)
    pub fn maximum_element_after_decrementing_and_rearranging(mut arr: Vec<i32>) -> i32 {
        arr.sort_unstable();
        arr.iter().fold(0, |prev, &x| prev + 1.min(x - prev))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        // [2,2,1,2,1] -> sorted [1,1,2,2,2] -> [1,1,2,2,2] max = 2
        assert_eq!(
            Solution::maximum_element_after_decrementing_and_rearranging(vec![2, 2, 1, 2, 1]),
            2
        );
    }

    #[test]
    fn test_example2() {
        // [100,1,1000] -> sorted [1,100,1000] -> [1,2,3] max = 3
        assert_eq!(
            Solution::maximum_element_after_decrementing_and_rearranging(vec![100, 1, 1000]),
            3
        );
    }

    #[test]
    fn test_example3() {
        // [1,2,3,4,5] already satisfies conditions, max = 5
        assert_eq!(
            Solution::maximum_element_after_decrementing_and_rearranging(vec![1, 2, 3, 4, 5]),
            5
        );
    }

    #[test]
    fn test_single_element() {
        assert_eq!(
            Solution::maximum_element_after_decrementing_and_rearranging(vec![1]),
            1
        );
        assert_eq!(
            Solution::maximum_element_after_decrementing_and_rearranging(vec![1000000000]),
            1
        );
    }

    #[test]
    fn test_all_same() {
        // [3,3,3,3] -> after ops [1,2,3,3] -> max = 3... but wait:
        // sorted [3,3,3,3]: prev=0 -> min(3,1)=1, min(3,2)=2, min(3,3)=3, min(3,4)=3 -> 3
        assert_eq!(
            Solution::maximum_element_after_decrementing_and_rearranging(vec![3, 3, 3, 3]),
            3
        );
    }

    #[test]
    fn test_large_gap() {
        // [1, 1000000000, 1000000000] -> sorted same -> prev=0->1, 1->2, 2->3 -> 3
        assert_eq!(
            Solution::maximum_element_after_decrementing_and_rearranging(vec![
                1,
                1_000_000_000,
                1_000_000_000
            ]),
            3
        );
    }

    #[test]
    fn test_decreasing_array() {
        // [5,4,3,2,1] -> sorted [1,2,3,4,5] -> max = 5
        assert_eq!(
            Solution::maximum_element_after_decrementing_and_rearranging(vec![5, 4, 3, 2, 1]),
            5
        );
    }
}
