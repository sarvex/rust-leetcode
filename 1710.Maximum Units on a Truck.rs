impl Solution {
    /// Greedy selection of boxes maximizing total units within truck capacity.
    ///
    /// # Intuition
    /// To maximize units, always pick boxes with the highest units-per-box first.
    /// Sort by units descending and greedily fill the truck.
    ///
    /// # Approach
    /// 1. Sort box types by units per box in descending order.
    /// 2. Iterate, taking as many boxes as capacity allows from each type.
    /// 3. Accumulate total units until the truck is full.
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(1) auxiliary (sort in place)
    pub fn maximum_units(mut box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        box_types.sort_unstable_by(|a, b| b[1].cmp(&a[1]));
        let mut remaining = truck_size;
        box_types.iter().fold(0, |units, bt| {
            let take = bt[0].min(remaining);
            remaining -= take;
            units + take * bt[1]
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        assert_eq!(
            Solution::maximum_units(vec![vec![1, 3], vec![2, 2], vec![3, 1]], 4),
            8
        );
    }

    #[test]
    fn test_example_two() {
        assert_eq!(
            Solution::maximum_units(vec![vec![5, 10], vec![2, 5], vec![4, 7], vec![3, 9]], 10),
            91
        );
    }
}
