impl Solution {
    /// Binary search for the minimum k such that total operations ≤ k².
    ///
    /// # Intuition
    /// For a fixed k, the minimum ops to make every element non-positive is
    /// sum of ceil(nums[i] / k). We want the smallest k with that sum ≤ k².
    ///
    /// # Approach
    /// Binary search on k. For each candidate, compute total ops in O(n).
    /// Upper bound analysis: worst case is n=10^5 elements all equal to 10^5,
    /// requiring k ≈ 2170 (where k³ ≈ 10^10). Using 3001 is safe.
    ///
    /// # Complexity
    /// - Time: O(n log K) where K ≈ 3000 is the search range.
    /// - Space: O(1)
    pub fn minimum_k(nums: Vec<i32>) -> i32 {
        let (mut lo, mut hi) = (1, 3001);
        while lo < hi {
            let k = lo + (hi - lo) / 2;
            let k2 = (k as i64) * (k as i64);
            let ops: i64 = nums.iter().map(|&x| ((x + k - 1) / k) as i64).sum();
            if ops <= k2 {
                hi = k;
            } else {
                lo = k + 1;
            }
        }
        lo
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::minimum_k(vec![3, 7, 5]), 3);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::minimum_k(vec![1]), 1);
    }

    #[test]
    fn test_single_large() {
        assert_eq!(Solution::minimum_k(vec![10]), 1);
    }

    #[test]
    fn test_two_elements() {
        // k=1: ops = 2+3=5 > 1; k=2: ceil(2/2)+ceil(3/2)=1+2=3 > 4? 3<=4, so k=2 works
        assert_eq!(Solution::minimum_k(vec![2, 3]), 2);
    }
}
