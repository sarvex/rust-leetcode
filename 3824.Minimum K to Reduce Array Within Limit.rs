impl Solution {
    /// Binary search for the minimum k such that total operations ≤ k².
    ///
    /// # Intuition
    /// For a fixed k, the minimum ops to make every element non-positive is
    /// sum of ceil(nums[i] / k). We want the smallest k with that sum ≤ k².
    ///
    /// # Approach
    /// For each k, total ops = Σ ceil(nums[i]/k). Binary search on k in [1, high]:
    /// if ops(k) ≤ k², k is valid and we try smaller k; otherwise we need larger k.
    /// Upper bound: when k ≥ max(nums), each element needs 1 op, so ops = n and
    /// we need k ≥ ceil(√n); thus a safe high is max(nums) or √n, we use 10^5+1.
    ///
    /// # Complexity
    /// - Time: O(n log M) with M = max(nums), from binary search and summing ops per k.
    /// - Space: O(1)
    pub fn minimum_k(nums: Vec<i32>) -> i32 {
        let max_val = *nums.iter().max().unwrap_or(&1);
        let n = nums.len();
        let sqrt_bound = (n as f64).sqrt().ceil() as i32 + 1;
        let high = max_val.max(sqrt_bound).max(1) + 1;

        let ops = |k: i32| {
            let k = k as i64;
            nums.iter()
                .map(|&x| ((x as i64) + k - 1) / k)
                .sum::<i64>()
        };

        let (mut lo, mut hi) = (1i32, high);
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if ops(mid) <= (mid as i64) * (mid as i64) {
                hi = mid;
            } else {
                lo = mid + 1;
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
