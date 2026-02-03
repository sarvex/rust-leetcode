use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

impl Solution {
    /// Greedy profit-first selection with category swaps.
    ///
    /// # Intuition
    /// Start with the `k` highest-profit items. If some categories repeat, swapping a low-profit
    /// duplicate for a new category can increase `distinct^2` enough to beat the profit loss.
    ///
    /// # Approach
    /// Sort items by profit descending and take the first `k`. Track seen categories in a set and
    /// push profits of duplicate-category items into a min-heap. The initial elegance is the sum
    /// plus `distinct^2`. Then scan remaining items in profit order: when an unseen category
    /// appears and there is a duplicate to replace, swap out the smallest duplicate profit, update
    /// the sum and distinct count, and refresh the best elegance.
    ///
    /// # Complexity
    /// - Time: O(n log n) for sorting and heap operations
    /// - Space: O(n) for the set and heap
    pub fn find_maximum_elegance(items: Vec<Vec<i32>>, k: i32) -> i64 {
        let mut items: Vec<(i64, i32)> = items
            .into_iter()
            .map(|entry| (entry[0] as i64, entry[1]))
            .collect();
        items.sort_by(|a, b| b.0.cmp(&a.0));

        let k = k as usize;
        let mut total_profit: i64 = 0;
        let mut seen_categories: HashSet<i32> = HashSet::with_capacity(k.saturating_mul(2) + 1);
        let mut duplicates: BinaryHeap<Reverse<i64>> = BinaryHeap::new();

        for (index, (profit, category)) in items.iter().enumerate() {
            if index >= k {
                break;
            }
            total_profit += *profit;
            if !seen_categories.insert(*category) {
                duplicates.push(Reverse(*profit));
            }
        }

        let mut distinct = seen_categories.len() as i64;
        let mut best = total_profit + distinct * distinct;

        for (profit, category) in items.iter().skip(k) {
            if seen_categories.contains(category) {
                continue;
            }
            let Some(Reverse(removed_profit)) = duplicates.pop() else {
                break;
            };
            total_profit += *profit - removed_profit;
            seen_categories.insert(*category);
            distinct += 1;
            best = best.max(total_profit + distinct * distinct);
        }

        best
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let items = vec![vec![3, 2], vec![5, 1], vec![10, 1]];
        assert_eq!(Solution::find_maximum_elegance(items, 2), 17);
    }

    #[test]
    fn example_two() {
        let items = vec![vec![3, 1], vec![3, 1], vec![2, 2], vec![5, 3]];
        assert_eq!(Solution::find_maximum_elegance(items, 3), 19);
    }

    #[test]
    fn example_three() {
        let items = vec![vec![1, 1], vec![2, 1], vec![3, 1]];
        assert_eq!(Solution::find_maximum_elegance(items, 3), 7);
    }

    #[test]
    fn swap_for_new_category() {
        let items = vec![vec![10, 1], vec![9, 1], vec![8, 2], vec![7, 3]];
        assert_eq!(Solution::find_maximum_elegance(items, 3), 34);
    }

    #[test]
    fn single_pick_uses_best_profit() {
        let items = vec![vec![5, 1], vec![4, 2], vec![3, 3]];
        assert_eq!(Solution::find_maximum_elegance(items, 1), 6);
    }
}
