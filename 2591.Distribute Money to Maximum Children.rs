impl Solution {
    /// Distribute money to maximize children receiving exactly 8 dollars.
    ///
    /// # Intuition
    /// Each child must get at least 1. After giving 1 to each child, distribute
    /// remaining in chunks of 7 (to reach 8 total). Special cases handle
    /// impossible distributions and avoiding 4-dollar totals.
    ///
    /// # Approach
    /// 1. If money < children, return -1
    /// 2. If money > children * 8, one child absorbs excess, so answer is children - 1
    /// 3. If remainder equals exactly 4 with one child left, that child gets 4 (forbidden),
    ///    so answer is children - 2
    /// 4. Otherwise, (money - children) / 7 children get exactly 8
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn dist_money(money: i32, children: i32) -> i32 {
        if money < children {
            return -1;
        }
        if money > children * 8 {
            return children - 1;
        }
        if money == children * 8 - 4 {
            return children - 2;
        }
        (money - children) / 7
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::dist_money(20, 3), 1);
    }

    #[test]
    fn test_insufficient() {
        assert_eq!(Solution::dist_money(2, 3), -1);
    }

    #[test]
    fn test_exact() {
        assert_eq!(Solution::dist_money(16, 2), 2);
    }

    #[test]
    fn test_excess() {
        assert_eq!(Solution::dist_money(17, 2), 1);
    }

    #[test]
    fn test_forbidden_four() {
        assert_eq!(Solution::dist_money(12, 2), 0);
    }
}
