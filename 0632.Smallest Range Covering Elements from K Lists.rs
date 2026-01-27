use std::collections::HashMap;

impl Solution {
    /// Finds the smallest range containing at least one element from each list.
    ///
    /// # Intuition
    /// Merge all elements with their list index, sort, then use a sliding
    /// window that expands until all lists are covered and shrinks to minimize
    /// the range.
    ///
    /// # Approach
    /// 1. Flatten all lists into (value, list_index) pairs and sort by value.
    /// 2. Sliding window: expand right to cover all k lists.
    /// 3. Shrink left while maintaining coverage, tracking the smallest range.
    ///
    /// # Complexity
    /// - Time: O(N log N) where N = total elements across all lists
    /// - Space: O(N)
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let k = nums.len();
        let mut pairs: Vec<(i32, usize)> = nums
            .iter()
            .enumerate()
            .flat_map(|(i, list)| list.iter().map(move |&v| (v, i)))
            .collect();
        pairs.sort_unstable();

        let mut best = vec![pairs[0].0, *&pairs.last().unwrap().0];
        let mut count: HashMap<usize, usize> = HashMap::new();
        let mut covered = 0;
        let mut left = 0;

        for right in 0..pairs.len() {
            let (val, idx) = pairs[right];
            let entry = count.entry(idx).or_insert(0);
            if *entry == 0 {
                covered += 1;
            }
            *entry += 1;

            while covered == k {
                let (lval, lidx) = pairs[left];
                let range = val - lval;
                let best_range = best[1] - best[0];
                if range < best_range || (range == best_range && lval < best[0]) {
                    best = vec![lval, val];
                }
                let e = count.get_mut(&lidx).unwrap();
                *e -= 1;
                if *e == 0 {
                    covered -= 1;
                }
                left += 1;
            }
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
            Solution::smallest_range(vec![
                vec![4, 10, 15, 24, 26],
                vec![0, 9, 12, 20],
                vec![5, 18, 22, 30],
            ]),
            vec![20, 24]
        );
    }

    #[test]
    fn test_single_lists() {
        assert_eq!(
            Solution::smallest_range(vec![vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3]]),
            vec![1, 1]
        );
    }
}
