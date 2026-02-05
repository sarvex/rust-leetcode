impl Solution {
    /// Partitions array into three subsequences to maximize XOR(A) + AND(B) + XOR(C)
    ///
    /// # Intuition
    /// For a fixed subset B, the remaining elements form set S to split between A and C.
    /// Since XOR(A) ^ XOR(C) = XOR(S), we have XOR(A) + XOR(C) = XOR(S) + 2*(XOR(A) & !XOR(S)).
    /// Maximizing XOR(A) & !XOR(S) reduces to maximizing XOR in a projected linear basis over GF(2).
    ///
    /// # Approach
    /// 1. Precompute AND, XOR, OR for all subsets via bitmask DP
    /// 2. Enumerate all 2^n subsets for B with pruning (upper bound: AND(B) + 2*OR(S) - XOR(S))
    /// 3. Build linear basis of remaining elements, project onto mask = !XOR(S)
    /// 4. Greedily maximize XOR from projected basis in O(B) time
    ///
    /// # Complexity
    /// - Time: O(2^n * n * B) where n is array length and B is bit width (30)
    /// - Space: O(2^n) for subset precomputation
    pub fn maximize_xor_and_xor(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let full = (1usize << n) - 1;
        let size = 1usize << n;

        let mut and_val = vec![0i32; size];
        let mut xor_val = vec![0i32; size];
        let mut or_val = vec![0i32; size];

        (1..size).for_each(|mask| {
            let i = mask.trailing_zeros() as usize;
            let rest = mask ^ (1 << i);
            xor_val[mask] = xor_val[rest] ^ nums[i];
            or_val[mask] = or_val[rest] | nums[i];
            and_val[mask] = match rest {
                0 => nums[i],
                _ => and_val[rest] & nums[i],
            };
        });

        (0..size).fold(0i64, |best, mask_b| {
            let mask_s = full ^ mask_b;
            let and_b = and_val[mask_b] as i64;

            if and_b + 2 * or_val[mask_s] as i64 - xor_val[mask_s] as i64 <= best {
                return best;
            }

            let xor_s = xor_val[mask_s];

            let mut basis = [0i32; 30];
            let mut s = mask_s;
            while s != 0 {
                let i = s.trailing_zeros() as usize;
                Self::insert_basis(&mut basis, nums[i]);
                s &= s - 1;
            }

            let m = !xor_s;
            let mut proj_basis = [0i32; 30];
            (0..30).rev().for_each(|j| {
                if basis[j] != 0 {
                    Self::insert_basis(&mut proj_basis, basis[j] & m);
                }
            });

            let max_extra = (0..30).rev().fold(0i32, |acc, j| match proj_basis[j] {
                0 => acc,
                v => acc.max(acc ^ v),
            });

            best.max(and_b + xor_s as i64 + 2 * max_extra as i64)
        })
    }

    #[inline]
    fn insert_basis(basis: &mut [i32; 30], val: i32) {
        let mut cur = val;
        for i in (0..30).rev() {
            if (cur >> i) & 1 == 0 {
                continue;
            }
            match basis[i] {
                0 => {
                    basis[i] = cur;
                    return;
                }
                v => cur ^= v,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_elements_simple() {
        assert_eq!(Solution::maximize_xor_and_xor(vec![2, 3]), 5);
    }

    #[test]
    fn three_elements_optimal_partition() {
        assert_eq!(Solution::maximize_xor_and_xor(vec![1, 3, 2]), 6);
    }

    #[test]
    fn four_elements_power_of_two_range() {
        assert_eq!(Solution::maximize_xor_and_xor(vec![2, 3, 6, 7]), 15);
    }

    #[test]
    fn four_elements_large_values() {
        assert_eq!(
            Solution::maximize_xor_and_xor(vec![902, 558, 609, 420]),
            2425
        );
    }

    #[test]
    fn five_elements_mixed() {
        assert_eq!(
            Solution::maximize_xor_and_xor(vec![386, 610, 2110, 2392, 347]),
            5789
        );
    }

    #[test]
    fn single_element_all_to_one_group() {
        assert_eq!(Solution::maximize_xor_and_xor(vec![5]), 5);
    }

    #[test]
    fn all_identical_elements() {
        assert_eq!(Solution::maximize_xor_and_xor(vec![4, 4, 4]), 12);
    }

    #[test]
    fn sequential_seventeen_elements() {
        let nums: Vec<i32> = (1..=17).collect();
        assert_eq!(Solution::maximize_xor_and_xor(nums), 76);
    }
}
