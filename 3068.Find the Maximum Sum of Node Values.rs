impl Solution {
    /// Maximum sum of node values after XOR-ing pairs along tree edges.
    ///
    /// # Intuition
    /// XOR-ing along an edge flips both endpoints. We can flip any even-sized subset
    /// of nodes by choosing appropriate edge paths. Track two states: best sum with
    /// an even number of flips (f0) and odd number of flips (f1).
    ///
    /// # Approach
    /// 1. Iterate through each node value maintaining two DP states.
    /// 2. f0 = best sum when an even count of nodes have been XOR-ed.
    /// 3. f1 = best sum when an odd count of nodes have been XOR-ed.
    /// 4. For each node, decide whether to XOR it or keep it, updating both states.
    /// 5. Return f0 (even flips form valid edge selections).
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn maximum_value_sum(nums: Vec<i32>, k: i32, _edges: Vec<Vec<i32>>) -> i64 {
        nums.iter()
            .fold((0i64, i64::MIN), |(f0, f1), &x| {
                let xv = (x ^ k) as i64;
                let xn = x as i64;
                (f0.max(f0 + xn).max(f1 + xv), f1.max(f1 + xn).max(f0 + xv))
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xor_improves_sum() {
        assert_eq!(
            Solution::maximum_value_sum(vec![1, 2, 1], 3, vec![vec![0, 1], vec![0, 2]]),
            6
        );
    }

    #[test]
    fn no_xor_is_optimal() {
        assert_eq!(
            Solution::maximum_value_sum(
                vec![7, 7, 7, 7, 7, 7],
                3,
                vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4], vec![0, 5]]
            ),
            42
        );
    }
}
