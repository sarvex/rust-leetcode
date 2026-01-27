// The rand7() API is already defined for you.
// fn rand7() -> i32;

impl Solution {
    /// Implements rand10() using rejection sampling with rand7().
    ///
    /// # Intuition
    /// Two calls to rand7() produce a uniform distribution over 49 outcomes.
    /// Reject values above 40 and map the rest to 1–10 uniformly.
    ///
    /// # Approach
    /// 1. Generate a value in [1, 49] using (rand7()-1)*7 + rand7().
    /// 2. If the value exceeds 40, retry.
    /// 3. Return (value % 10) + 1.
    ///
    /// # Complexity
    /// - Time: O(1) expected — each iteration succeeds with probability 40/49
    /// - Space: O(1)
    pub fn rand10() -> i32 {
        loop {
            let val = (rand7() - 1) * 7 + rand7();
            if val <= 40 {
                return (val % 10) + 1;
            }
        }
    }
}
