impl Solution {
    /// Count subarrays whose sum equals at least one element in the subarray.
    ///
    /// # Intuition
    /// A subarray is centered iff its sum appears as some element in that same
    /// subarray. Single-element subarrays are always centered; for longer ones
    /// we check sum ∈ elements.
    ///
    /// # Approach
    /// For each start index `i`, extend with end `j` from `i` to `n-1`,
    /// maintaining running sum and a fixed-capacity set of elements. Use
    /// epoch-based clearing (no realloc) and linear probing for cache-friendly
    /// lookups. Count when `sum` is in the element set.
    ///
    /// # Complexity
    /// - Time: O(n²)
    /// - Space: O(1) — fixed-size set (CAP slots)
    pub fn centered_subarrays(nums: Vec<i32>) -> i32 {
        const CAP: usize = 2048;
        const MASK: usize = CAP - 1;

        #[inline]
        fn hash_i32(x: i32) -> usize {
            let u = x as u32 as usize;
            u.wrapping_mul(0x9E37_79B1) & MASK
        }

        struct FastSet {
            keys: Vec<i32>,
            stamp: Vec<u32>,
            epoch: u32,
        }

        impl FastSet {
            #[inline]
            fn new() -> Self {
                Self { keys: vec![0; CAP], stamp: vec![0; CAP], epoch: 0 }
            }

            #[inline]
            fn next_epoch(&mut self) {
                self.epoch += 1;
            }

            #[inline]
            fn insert(&mut self, x: i32) {
                let mut idx = hash_i32(x);
                let e = self.epoch;
                loop {
                    unsafe {
                        // SAFETY: idx is in [0, MASK] (CAP is power of two, linear probing keeps idx & MASK).
                        if *self.stamp.get_unchecked(idx) != e {
                            *self.stamp.get_unchecked_mut(idx) = e;
                            *self.keys.get_unchecked_mut(idx) = x;
                            return;
                        }
                        if *self.keys.get_unchecked(idx) == x {
                            return;
                        }
                    }
                    idx = (idx + 1) & MASK;
                }
            }

            #[inline]
            fn contains(&self, x: i32) -> bool {
                let mut idx = hash_i32(x);
                let e = self.epoch;
                loop {
                    unsafe {
                        // SAFETY: idx in [0, MASK] as above.
                        if *self.stamp.get_unchecked(idx) != e {
                            return false;
                        }
                        if *self.keys.get_unchecked(idx) == x {
                            return true;
                        }
                    }
                    idx = (idx + 1) & MASK;
                }
            }
        }

        let n = nums.len();
        let mut ans: i32 = 0;
        let mut set = FastSet::new();

        for i in 0..n {
            set.next_epoch();
            let mut sum: i32 = 0;
            for j in i..n {
                // SAFETY: j in [i, n) and i in [0, n], so j < n.
                let v = unsafe { *nums.get_unchecked(j) };
                sum += v;
                set.insert(v);
                if set.contains(sum) {
                    ans += 1;
                }
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(Solution::centered_subarrays(vec![-1, 1, 0]), 5);
    }

    #[test]
    fn test_example2() {
        assert_eq!(Solution::centered_subarrays(vec![2, -3]), 2);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::centered_subarrays(vec![7]), 1);
    }

    #[test]
    fn test_two_same() {
        assert_eq!(Solution::centered_subarrays(vec![1, 1]), 2);
    }

    #[test]
    fn test_sum_in_window() {
        assert_eq!(Solution::centered_subarrays(vec![1, 0]), 3);
    }
}
