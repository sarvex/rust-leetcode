impl Solution {
    /// Greedy ordering of cuts from largest cost to smallest.
    ///
    /// # Intuition
    /// Each cut cost is multiplied by the number of pieces in the other direction. To minimize
    /// the total, we want large costs to be multiplied by small piece counts.
    ///
    /// # Approach
    /// Sort horizontal and vertical cut costs in descending order. Track the current number of
    /// horizontal and vertical pieces. Repeatedly take the larger next cost, add cost * opposite
    /// pieces, and increment the corresponding piece count. Finish any remaining cuts.
    ///
    /// # Complexity
    /// - Time: O((m + n) log(m + n)) for sorting
    /// - Space: O(1) extra (in-place sort)
    pub fn minimum_cost(
        m: i32,
        n: i32,
        horizontal_cut: Vec<i32>,
        vertical_cut: Vec<i32>,
    ) -> i64 {
        let _ = (m, n);
        let mut horizontal = horizontal_cut;
        let mut vertical = vertical_cut;

        horizontal.sort_unstable_by(|a, b| b.cmp(a));
        vertical.sort_unstable_by(|a, b| b.cmp(a));

        let mut horizontal_pieces: i64 = 1;
        let mut vertical_pieces: i64 = 1;
        let mut total: i64 = 0;

        let mut h = 0usize;
        let mut v = 0usize;

        while h < horizontal.len() && v < vertical.len() {
            if horizontal[h] >= vertical[v] {
                total += horizontal[h] as i64 * vertical_pieces;
                horizontal_pieces += 1;
                h += 1;
            } else {
                total += vertical[v] as i64 * horizontal_pieces;
                vertical_pieces += 1;
                v += 1;
            }
        }

        while h < horizontal.len() {
            total += horizontal[h] as i64 * vertical_pieces;
            horizontal_pieces += 1;
            h += 1;
        }

        while v < vertical.len() {
            total += vertical[v] as i64 * horizontal_pieces;
            vertical_pieces += 1;
            v += 1;
        }

        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let m = 3;
        let n = 2;
        let horizontal_cut = vec![1, 3];
        let vertical_cut = vec![5];
        assert_eq!(Solution::minimum_cost(m, n, horizontal_cut, vertical_cut), 13);
    }

    #[test]
    fn test_example_2() {
        let m = 2;
        let n = 2;
        let horizontal_cut = vec![7];
        let vertical_cut = vec![4];
        assert_eq!(Solution::minimum_cost(m, n, horizontal_cut, vertical_cut), 15);
    }

    #[test]
    fn test_single_cell() {
        let m = 1;
        let n = 1;
        let horizontal_cut = vec![];
        let vertical_cut = vec![];
        assert_eq!(Solution::minimum_cost(m, n, horizontal_cut, vertical_cut), 0);
    }

    #[test]
    fn test_only_vertical_cuts() {
        let m = 1;
        let n = 4;
        let horizontal_cut = vec![];
        let vertical_cut = vec![2, 1, 3];
        assert_eq!(Solution::minimum_cost(m, n, horizontal_cut, vertical_cut), 6);
    }

    #[test]
    fn test_greedy_advantage() {
        let m = 2;
        let n = 3;
        let horizontal_cut = vec![2];
        let vertical_cut = vec![1, 3];
        assert_eq!(Solution::minimum_cost(m, n, horizontal_cut, vertical_cut), 9);
    }
}
