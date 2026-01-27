impl Solution {
    /// Determine who holds the pillow after a given time.
    ///
    /// # Intuition
    /// The pillow bounces back and forth. Each full pass takes n-1 seconds.
    /// The position within the current pass determines the holder.
    ///
    /// # Approach
    /// 1. Compute full passes and remainder: time / (n-1) and time % (n-1)
    /// 2. If even number of passes, position is 1 + remainder (going right)
    /// 3. If odd, position is n - remainder (going left)
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn pass_the_pillow(n: i32, time: i32) -> i32 {
        let cycle = n - 1;
        let full = time / cycle;
        let rem = time % cycle;

        if full % 2 == 0 {
            1 + rem
        } else {
            n - rem
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::pass_the_pillow(4, 5), 2);
    }

    #[test]
    fn test_at_start() {
        assert_eq!(Solution::pass_the_pillow(3, 0), 1);
    }

    #[test]
    fn test_full_cycle() {
        assert_eq!(Solution::pass_the_pillow(3, 4), 1);
    }

    #[test]
    fn test_end_of_pass() {
        assert_eq!(Solution::pass_the_pillow(4, 3), 4);
    }
}
