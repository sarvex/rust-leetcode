impl Solution {
    /// Rearrange digits to form the smallest (or largest-magnitude negative) value.
    ///
    /// # Intuition
    /// For positive numbers, place digits ascending with the smallest non-zero digit
    /// first. For negatives, place digits descending to maximize magnitude.
    ///
    /// # Approach
    /// 1. Count digit frequencies from the absolute value.
    /// 2. If negative, build the number with digits in descending order.
    /// 3. If positive and zeros exist, place the smallest non-zero digit first,
    ///    then append all digits (including zeros) in ascending order.
    ///
    /// # Complexity
    /// - Time: O(d) where d is the number of digits
    /// - Space: O(1) — fixed 10-element frequency array
    pub fn smallest_number(num: i64) -> i64 {
        let negative = num < 0;
        let mut val = num.abs();
        let mut count = [0u32; 10];

        while val > 0 {
            count[(val % 10) as usize] += 1;
            val /= 10;
        }

        if negative {
            let result = (0..10).rev().fold(0i64, |acc, d| {
                (0..count[d]).fold(acc, |a, _| a * 10 + d as i64)
            });
            return -result;
        }

        let mut result = 0i64;
        if count[0] > 0 {
            if let Some(first) = (1..10).find(|&d| count[d] > 0) {
                count[first] -= 1;
                result = first as i64;
            }
        }

        for d in 0..10 {
            for _ in 0..count[d] {
                result = result * 10 + d as i64;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_with_zeros() {
        assert_eq!(Solution::smallest_number(310), 103);
    }

    #[test]
    fn negative_number() {
        assert_eq!(Solution::smallest_number(-7605), -7650);
    }

    #[test]
    fn zero_input() {
        assert_eq!(Solution::smallest_number(0), 0);
    }
}
