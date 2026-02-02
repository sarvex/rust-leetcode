impl Solution {
    /// Fenwick difference tracking for greater-count comparisons.
    ///
    /// # Intuition
    /// We only need counts of elements greater than each value in both arrays,
    /// which can be derived from prefix counts in a frequency structure.
    ///
    /// # Approach
    /// Coordinate-compress `nums` so each value maps to `1..=m`, ordered by
    /// descending value. Maintain a single Fenwick tree storing the frequency
    /// difference `count(arr1) - count(arr2)` at each rank. For each value,
    /// the prefix sum over ranks greater than it gives the difference in
    /// greater counts, which directly determines the choice.
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(n)
    pub fn result_array(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut sorted = nums.clone();
        sorted.sort_unstable();
        sorted.dedup();

        let mut arr1 = Vec::with_capacity(n);
        let mut arr2 = Vec::with_capacity(n);
        let mut diff = Fenwick::new(sorted.len());

        let first = nums[0];
        let second = nums[1];
        let rank = |value: i32| -> usize {
            let idx = sorted.binary_search(&value).expect("value must exist");
            sorted.len() - idx
        };
        arr1.push(first);
        arr2.push(second);
        diff.add(rank(first), 1);
        diff.add(rank(second), -1);

        for &value in nums.iter().skip(2) {
            let r = rank(value);
            let greater_diff = diff.prefix_sum(r.saturating_sub(1));
            let choose_arr1 = greater_diff > 0 || (greater_diff == 0 && arr1.len() <= arr2.len());

            if choose_arr1 {
                arr1.push(value);
                diff.add(r, 1);
            } else {
                arr2.push(value);
                diff.add(r, -1);
            }
        }

        arr1.extend(arr2);
        arr1
    }
}

#[derive(Debug, Clone)]
struct Fenwick {
    tree: Vec<i32>,
}

impl Fenwick {
    fn new(size: usize) -> Self {
        Self {
            tree: vec![0; size + 1],
        }
    }

    fn add(&mut self, mut idx: usize, delta: i32) {
        let n = self.tree.len();
        while idx < n {
            self.tree[idx] += delta;
            idx += idx & idx.wrapping_neg();
        }
    }

    fn prefix_sum(&self, mut idx: usize) -> i32 {
        let mut sum = 0i32;
        while idx > 0 {
            sum += self.tree[idx];
            idx &= idx - 1;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::result_array(vec![2, 1, 3, 3]), vec![2, 3, 1, 3]);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::result_array(vec![5, 14, 3, 1, 2]),
            vec![5, 3, 1, 2, 14]
        );
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::result_array(vec![3, 3, 3, 3]), vec![3, 3, 3, 3]);
    }

    #[test]
    fn test_length_tie_goes_to_arr1() {
        assert_eq!(Solution::result_array(vec![1, 2, 3]), vec![1, 3, 2]);
    }

    #[test]
    fn test_strictly_greater_wins() {
        assert_eq!(Solution::result_array(vec![4, 1, 2, 3]), vec![4, 2, 3, 1]);
    }
}
