impl Solution {
    /// Fibonacci recurrence for counting distinct stair-climbing paths.
    ///
    /// # Intuition
    /// Reaching step `n` requires arriving from step `n-1` (one step) or step
    /// `n-2` (two steps). The total ways thus follows the Fibonacci recurrence,
    /// making bottom-up iteration with two variables optimal.
    ///
    /// # Approach
    /// Initialize two accumulators representing `f(n-2)` and `f(n-1)`. Iterate
    /// from 1 to `n`, updating them in lockstep. The final value of the second
    /// accumulator is the answer.
    ///
    /// # Complexity
    /// - Time: O(n) — single linear pass
    /// - Space: O(1) — two scalar accumulators
    pub fn climb_stairs(n: i32) -> i32 {
        (1..n).fold((1, 1), |(prev, curr), _| (curr, prev + curr)).1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_step() {
        assert_eq!(Solution::climb_stairs(1), 1);
    }

    #[test]
    fn two_steps() {
        assert_eq!(Solution::climb_stairs(2), 2);
    }

    #[test]
    fn three_steps() {
        assert_eq!(Solution::climb_stairs(3), 3);
    }

    #[test]
    fn five_steps() {
        assert_eq!(Solution::climb_stairs(5), 8);
    }

    #[test]
    fn ten_steps() {
        assert_eq!(Solution::climb_stairs(10), 89);
    }

    #[test]
    fn large_input() {
        assert_eq!(Solution::climb_stairs(45), 1836311903);
    }
}
