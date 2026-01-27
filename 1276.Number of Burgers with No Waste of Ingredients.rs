impl Solution {
    /// Linear algebra solution for the burger ingredient system.
    ///
    /// # Intuition
    /// With x jumbo burgers (4 slices) and y small burgers (2 slices), the
    /// system `4x + 2y = tomato, x + y = cheese` solves to
    /// `x = (tomato - 2*cheese) / 2` and `y = cheese - x`. Solutions must be
    /// non-negative integers.
    ///
    /// # Approach
    /// 1. Compute `k = 4*cheese - tomato` (equals `2*y`)
    /// 2. Check divisibility by 2 and non-negativity of both x and y
    /// 3. Return `[x, y]` or empty vector if impossible
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn num_of_burgers(tomato_slices: i32, cheese_slices: i32) -> Vec<i32> {
        let k = 4 * cheese_slices - tomato_slices;
        let y = k / 2;
        let x = cheese_slices - y;
        if k % 2 != 0 || y < 0 || x < 0 {
            vec![]
        } else {
            vec![x, y]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_solution() {
        assert_eq!(Solution::num_of_burgers(16, 7), vec![1, 6]);
    }

    #[test]
    fn no_solution_odd() {
        assert_eq!(Solution::num_of_burgers(17, 4), vec![]);
    }

    #[test]
    fn no_solution_negative() {
        assert_eq!(Solution::num_of_burgers(4, 17), vec![]);
    }

    #[test]
    fn all_jumbo() {
        assert_eq!(Solution::num_of_burgers(0, 0), vec![0, 0]);
    }

    #[test]
    fn large_values() {
        assert_eq!(Solution::num_of_burgers(2, 1), vec![0, 1]);
    }
}
