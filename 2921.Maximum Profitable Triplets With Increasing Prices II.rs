struct BinaryIndexedTree {
    n: usize,
    c: Vec<i32>,
}

impl BinaryIndexedTree {
    fn new(n: usize) -> Self {
        Self {
            n,
            c: vec![0; n + 1],
        }
    }

    fn update(&mut self, mut x: usize, v: i32) {
        while x <= self.n {
            self.c[x] = self.c[x].max(v);
            x += x & x.wrapping_neg();
        }
    }

    fn query(&self, mut x: usize) -> i32 {
        let mut mx = 0;
        while x > 0 {
            mx = mx.max(self.c[x]);
            x -= x & x.wrapping_neg();
        }
        mx
    }
}

impl Solution {
    /// Finds the maximum profit from a triplet with strictly increasing prices using BIT.
    ///
    /// # Intuition
    /// Use two Binary Indexed Trees to efficiently query the best left and right
    /// profits. The forward BIT tracks maximum profit for prices less than the
    /// current, and the reverse BIT tracks maximum profit for prices greater.
    ///
    /// # Approach
    /// 1. Build a forward BIT: for each index, query max profit among smaller prices,
    ///    then update with current profit.
    /// 2. Build a reverse BIT: traverse backwards with complemented indices.
    /// 3. Combine: for each middle element, sum left + own profit + right.
    ///
    /// # Complexity
    /// - Time: O(n log M) where M is the maximum price
    /// - Space: O(n + M) for BIT and auxiliary arrays
    pub fn max_profit(prices: Vec<i32>, profits: Vec<i32>) -> i32 {
        let n = prices.len();
        let m = prices.iter().copied().max().unwrap_or(0) as usize;

        let mut left = vec![0i32; n];
        let mut right = vec![0i32; n];

        let mut tree1 = BinaryIndexedTree::new(m + 1);
        let mut tree2 = BinaryIndexedTree::new(m + 1);

        for i in 0..n {
            let x = prices[i] as usize;
            left[i] = tree1.query(x - 1);
            tree1.update(x, profits[i]);
        }

        for i in (0..n).rev() {
            let x = (m as i32 + 1 - prices[i]) as usize;
            right[i] = tree2.query(x - 1);
            tree2.update(x, profits[i]);
        }

        (0..n)
            .filter(|&i| left[i] > 0 && right[i] > 0)
            .map(|i| left[i] + profits[i] + right[i])
            .max()
            .unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profitable_triplet() {
        assert_eq!(
            Solution::max_profit(vec![10, 2, 3, 4], vec![100, 2, 7, 10]),
            19
        );
    }

    #[test]
    fn test_no_valid_triplet() {
        assert_eq!(Solution::max_profit(vec![3, 2, 1], vec![1, 2, 3]), -1);
    }
}
