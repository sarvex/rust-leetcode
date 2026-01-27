impl Solution {
    /// Checks if point (targetX, targetY) is reachable from (1, 1).
    ///
    /// # Intuition
    /// The allowed operations consist of two types:
    /// - Subtraction: (x, y-x) or (x-y, y) — essentially GCD operations
    /// - Doubling: (2x, y) or (x, 2y) — introduces powers of 2
    ///
    /// # Approach
    /// Starting from (1, 1), subtraction operations preserve the GCD while doubling
    /// can only multiply coordinates by powers of 2. Therefore, gcd(targetX, targetY)
    /// must be a power of 2 (including 2^0 = 1) for any point to be reachable.
    ///
    /// # Complexity
    /// - Time: O(log(min(targetX, targetY))) for GCD computation
    /// - Space: O(1)
    pub fn is_reachable(target_x: i32, target_y: i32) -> bool {
        let g = Self::gcd(target_x, target_y);
        g & (g - 1) == 0
    }

    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unreachable() {
        assert!(!Solution::is_reachable(6, 9));
    }

    #[test]
    fn test_reachable() {
        assert!(Solution::is_reachable(4, 7));
    }

    #[test]
    fn test_origin_adjacent() {
        assert!(Solution::is_reachable(1, 1));
    }

    #[test]
    fn test_power_of_two_coordinates() {
        assert!(Solution::is_reachable(8, 16));
    }

    #[test]
    fn test_coprime() {
        assert!(Solution::is_reachable(3, 5));
    }

    #[test]
    fn test_large_coordinates() {
        assert!(Solution::is_reachable(1_000_000_000, 1));
    }

    #[test]
    fn test_gcd_is_four() {
        assert!(Solution::is_reachable(8, 12));
    }

    #[test]
    fn test_gcd_is_six() {
        assert!(!Solution::is_reachable(12, 18));
    }
}
