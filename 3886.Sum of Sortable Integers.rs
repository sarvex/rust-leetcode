impl Solution {
    /// Sums all divisors k of n for which cyclic rotation of k-chunks can sort the array.
    ///
    /// # Intuition
    /// A chunk admits a valid rotation iff it has at most one circular descent.  When
    /// exactly one descent exists, its position pins the chunk's min and max after
    /// rotation—no range-query structure is needed.
    ///
    /// # Approach
    /// Build a single next-descent array: `nxt[i]` = smallest `j ≥ i` with
    /// `nums[j] > nums[j+1]`, or `n` if none.  For each chunk `[s, e]`, two lookups
    /// into `nxt` determine the drop count (0 / 1 / 2+) and the descent position in
    /// O(1).  Divisors are enumerated in O(√n) without allocation.
    ///
    /// # Complexity
    /// - Time: O(n + σ(n)), where σ(n) is the sum-of-divisors function
    /// - Space: O(n)
    pub fn sortable_integers(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        // nxt[i] = min j >= i where nums[j] > nums[j+1], or n if none
        let mut nxt = vec![n; n];
        for i in (0..n.saturating_sub(1)).rev() {
            nxt[i] = if nums[i] > nums[i + 1] { i } else { nxt[i + 1] };
        }

        let check = |k: usize| -> bool {
            let mut prev_max = i32::MIN;
            let mut s = 0;
            while s < n {
                let e = s + k - 1;
                let d = nxt[s];

                let (gmin, gmax) = if d + 1 > e {
                    (nums[s], nums[e])
                } else if nxt[d + 1] + 1 <= e {
                    return false;
                } else {
                    if nums[e] > nums[s] {
                        return false;
                    }
                    (nums[d + 1], nums[d])
                };

                if prev_max > gmin {
                    return false;
                }
                prev_max = gmax;
                s += k;
            }
            true
        };

        let mut result = 0i32;
        let mut i = 1;
        while i * i <= n {
            if n % i == 0 {
                if check(i) {
                    result += i as i32;
                }
                let pair = n / i;
                if pair != i && check(pair) {
                    result += pair as i32;
                }
            }
            i += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_rotation() {
        assert_eq!(Solution::sortable_integers(vec![3, 1, 2]), 3);
    }

    #[test]
    fn test_no_sortable() {
        assert_eq!(Solution::sortable_integers(vec![7, 6, 5]), 0);
    }

    #[test]
    fn test_already_sorted() {
        assert_eq!(Solution::sortable_integers(vec![5, 8]), 3);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::sortable_integers(vec![5]), 1);
    }

    #[test]
    fn test_all_equal() {
        assert_eq!(Solution::sortable_integers(vec![3, 3, 3]), 4);
    }

    #[test]
    fn test_four_elements_sorted() {
        assert_eq!(Solution::sortable_integers(vec![1, 2, 3, 4]), 7);
    }

    #[test]
    fn test_pairwise_swapped() {
        assert_eq!(Solution::sortable_integers(vec![2, 1, 4, 3]), 2);
    }
}
