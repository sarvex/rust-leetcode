impl Solution {
    /// Split digits into two numbers with minimum sum.
    ///
    /// # Intuition
    /// Distributing smaller digits across both numbers (alternating) minimizes
    /// the total. Sorting digits ensures smallest digits go to higher place values.
    ///
    /// # Approach
    /// 1. Count digit frequencies
    /// 2. Distribute digits alternately between two numbers (smallest first)
    /// 3. Return the sum
    ///
    /// # Complexity
    /// - Time: O(d) where d is the number of digits
    /// - Space: O(1)
    pub fn split_num(mut num: i32) -> i32 {
        let mut cnt = [0usize; 10];
        let mut n = 0;

        while num != 0 {
            cnt[(num as usize) % 10] += 1;
            num /= 10;
            n += 1;
        }

        let mut ans = [0i32; 2];
        let mut j = 0;
        for i in 0..n {
            while cnt[j] == 0 {
                j += 1;
            }
            cnt[j] -= 1;
            ans[i & 1] = ans[i & 1] * 10 + (j as i32);
        }

        ans[0] + ans[1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::split_num(4325), 59);
    }

    #[test]
    fn test_with_zeros() {
        assert_eq!(Solution::split_num(687), 75);
    }

    #[test]
    fn test_two_digits() {
        assert_eq!(Solution::split_num(10), 1);
    }
}
