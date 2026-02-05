use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    /// Minimum pair removal to sort array using min-heap with lazy deletion.
    ///
    /// # Intuition
    /// Use min-heap with lazy deletion via sum comparison. Staleness detected by
    /// checking if stored sum matches current adjacent values.
    ///
    /// # Approach
    /// 1. Encode linked list in nexts/prevs arrays, use -1 as sentinel for removed
    /// 2. Min-heap with (sum, index), staleness via sum mismatch
    /// 3. Track inversion count, terminate when zero
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(n)
    #[inline]
    pub fn minimum_pair_removal(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n <= 1 {
            return 0;
        }

        let mut vals: Vec<i64> = nums.iter().map(|&x| x as i64).collect();
        let mut nexts: Vec<i32> = (1..=n as i32).collect();
        let mut prevs: Vec<i32> = (-1..n as i32 - 1).collect();
        nexts[n - 1] = -1;

        let mut unsorted_cnt = 0i32;
        let mut heap: BinaryHeap<Reverse<(i64, u32)>> = BinaryHeap::with_capacity(n * 2);

        unsorted_cnt = vals.windows(2).filter(|w| w[0] > w[1]).count() as i32;
        vals.windows(2).enumerate().for_each(|(i, w)| {
            heap.push(Reverse((w[0] + w[1], i as u32)));
        });

        if unsorted_cnt == 0 {
            return 0;
        }

        let mut moves = 0i32;

        while let Some(Reverse((sum, ui))) = heap.pop() {
            let u = ui as usize;
            let v = nexts[u];

            if v < 0 {
                continue;
            }

            let v = v as usize;
            let (val_u, val_v) = (vals[u], vals[v]);

            if val_u + val_v != sum {
                continue;
            }

            let p = prevs[u];
            let nv = nexts[v];

            moves += 1;

            // Remove old inversions
            if p >= 0 {
                unsorted_cnt -= (vals[p as usize] > val_u) as i32;
            }
            unsorted_cnt -= (val_u > val_v) as i32;
            if nv >= 0 {
                unsorted_cnt -= (val_v > vals[nv as usize]) as i32;
            }

            // Merge v into u
            vals[u] = sum;
            nexts[u] = nv;
            if nv >= 0 {
                prevs[nv as usize] = u as i32;
            }
            nexts[v] = -1;

            // Add new inversions
            if p >= 0 {
                let val_p = vals[p as usize];
                unsorted_cnt += (val_p > sum) as i32;
                heap.push(Reverse((val_p + sum, p as u32)));
            }
            if nv >= 0 {
                let val_nv = vals[nv as usize];
                unsorted_cnt += (sum > val_nv) as i32;
                heap.push(Reverse((sum + val_nv, ui)));
            }

            if unsorted_cnt == 0 {
                break;
            }
        }

        moves
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::minimum_pair_removal(vec![5, 2, 3, 1]), 2);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::minimum_pair_removal(vec![1, 2, 2]), 0);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::minimum_pair_removal(vec![1]), 0);
    }

    #[test]
    fn test_two_elements_sorted() {
        assert_eq!(Solution::minimum_pair_removal(vec![1, 2]), 0);
    }

    #[test]
    fn test_two_elements_unsorted() {
        assert_eq!(Solution::minimum_pair_removal(vec![2, 1]), 1);
    }

    #[test]
    fn test_descending() {
        assert_eq!(Solution::minimum_pair_removal(vec![4, 3, 2, 1]), 2);
    }

    #[test]
    fn test_negative_numbers() {
        assert_eq!(Solution::minimum_pair_removal(vec![1, -1, 2]), 1);
    }
}
