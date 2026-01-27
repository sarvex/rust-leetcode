impl Solution {
    /// Build divisibility array using running modular arithmetic.
    ///
    /// # Intuition
    /// Track the running number mod m digit by digit to avoid overflow.
    /// If the remainder is 0, the prefix is divisible by m.
    ///
    /// # Approach
    /// 1. Iterate over bytes of the string
    /// 2. Maintain running remainder: x = (x * 10 + digit) % m
    /// 3. Output 1 if remainder is 0, else 0
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) for output
    pub fn divisibility_array(word: String, m: i32) -> Vec<i32> {
        let m = m as i64;
        let mut x = 0i64;

        word.as_bytes()
            .iter()
            .map(|&c| {
                x = (x * 10 + i64::from(c - b'0')) % m;
                i32::from(x == 0)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::divisibility_array("998244353".into(), 3),
            vec![1, 1, 0, 0, 0, 1, 1, 0, 0]
        );
    }

    #[test]
    fn test_no_divisible() {
        assert_eq!(
            Solution::divisibility_array("1010".into(), 10),
            vec![0, 1, 0, 1]
        );
    }
}
