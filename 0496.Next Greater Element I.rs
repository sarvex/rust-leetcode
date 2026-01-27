use std::collections::HashMap;

impl Solution {
    /// Finds the next greater element for each query using a monotonic stack.
    ///
    /// # Intuition
    /// Process nums2 from right to left with a decreasing stack. For each
    /// element, the stack top (if any) is the next greater element.
    ///
    /// # Approach
    /// 1. Traverse nums2 in reverse, maintaining a decreasing stack.
    /// 2. Map each element to its next greater element.
    /// 3. Look up each nums1 element in the map.
    ///
    /// # Complexity
    /// - Time: O(m + n)
    /// - Space: O(n)
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut stack = Vec::new();
        let mut map = HashMap::with_capacity(nums2.len());
        for &x in nums2.iter().rev() {
            while stack.last().map_or(false, |&top| top <= x) {
                stack.pop();
            }
            if let Some(&top) = stack.last() {
                map.insert(x, top);
            }
            stack.push(x);
        }
        nums1.iter().map(|x| *map.get(x).unwrap_or(&-1)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2]),
            vec![-1, 3, -1]
        );
    }

    #[test]
    fn test_all_found() {
        assert_eq!(
            Solution::next_greater_element(vec![2, 4], vec![1, 2, 3, 4]),
            vec![3, -1]
        );
    }
}
