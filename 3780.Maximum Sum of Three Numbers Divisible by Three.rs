impl Solution {
    /// Track top 3 elements per remainder class in a single pass (no sort).
    ///
    /// # Intuition
    /// A sum of three numbers is divisible by 3 iff the sum of their remainders mod 3 is 0.
    /// Valid triplets are: (0,0,0), (1,1,1), (2,2,2), or (0,1,2). We only need the largest
    /// 3 elements from each remainder class.
    ///
    /// # Approach
    /// Maintain (1st, 2nd, 3rd) largest per remainder in one pass with insertion-style
    /// updates. Then take the max of: sum of top 3 from r0, from r1, from r2, or one from each.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        let mut r0 = (0, 0, 0);
        let mut r1 = (0, 0, 0);
        let mut r2 = (0, 0, 0);
        for n in nums.iter().copied() {
            let z = match n % 3 {
                0 => &mut r0,
                1 => &mut r1,
                2 => &mut r2,
                _ => unreachable!(),
            };
            if n > z.0 {
                *z = (n, z.0, z.1);
            } else if n > z.1 {
                *z = (z.0, n, z.1);
            } else if n > z.2 {
                *z = (z.0, z.1, n);
            }
        }
        fn calc(i1: i32, i2: i32, i3: i32) -> i32 {
            if i1 > 0 && i2 > 0 && i3 > 0 {
                i1 + i2 + i3
            } else {
                0
            }
        }
        calc(r0.0, r0.1, r0.2)
            .max(calc(r1.0, r1.1, r1.2))
            .max(calc(r2.0, r2.1, r2.2))
            .max(calc(r0.0, r1.0, r2.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(Solution::maximum_sum(vec![4, 2, 3, 1]), 9);
    }

    #[test]
    fn test_example2() {
        assert_eq!(Solution::maximum_sum(vec![2, 1, 5]), 0);
    }

    #[test]
    fn test_three_mod_zero() {
        assert_eq!(Solution::maximum_sum(vec![3, 6, 9]), 18);
    }

    #[test]
    fn test_one_each_residue() {
        assert_eq!(Solution::maximum_sum(vec![3, 4, 5]), 12);
    }

    #[test]
    fn test_three_mod_one() {
        assert_eq!(Solution::maximum_sum(vec![1, 4, 7]), 12);
    }

    #[test]
    fn test_three_mod_two() {
        assert_eq!(Solution::maximum_sum(vec![2, 5, 8]), 15);
    }
}
