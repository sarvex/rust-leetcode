impl Solution {
    /// Binary search to find elements without close neighbors.
    ///
    /// # Intuition
    /// Sort arr2, then for each element in arr1, binary search for the nearest
    /// element in arr2. If no element in arr2 is within distance d, count it.
    ///
    /// # Approach
    /// 1. Sort arr2
    /// 2. For each x in arr1, find insertion point of `x - d` in arr2
    /// 3. If the element at that position exceeds `x + d`, no close neighbor exists
    ///
    /// # Complexity
    /// - Time: O(n log n + m log n) for sorting and binary searches
    /// - Space: O(1) auxiliary (excluding sort)
    pub fn find_the_distance_value(arr1: Vec<i32>, mut arr2: Vec<i32>, d: i32) -> i32 {
        arr2.sort_unstable();
        arr1.iter()
            .filter(|&&x| {
                let pos = arr2.partition_point(|&v| v < x - d);
                pos == arr2.len() || arr2[pos] > x + d
            })
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_distance() {
        assert_eq!(
            Solution::find_the_distance_value(vec![4, 5, 8], vec![10, 9, 1, 8], 2),
            2
        );
    }

    #[test]
    fn all_far() {
        assert_eq!(
            Solution::find_the_distance_value(vec![1, 4, 2, 3], vec![-4, -3, 6, 10, 20, 30], 3),
            2
        );
    }

    #[test]
    fn all_close() {
        assert_eq!(
            Solution::find_the_distance_value(vec![2, 1, 100, 3], vec![-5, -2, 10, -3, 7], 6),
            1
        );
    }
}
