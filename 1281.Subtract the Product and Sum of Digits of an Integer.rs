impl Solution {
    /// Digit extraction with simultaneous product and sum accumulation.
    ///
    /// # Intuition
    /// Extract digits one by one using modular arithmetic, accumulating the
    /// running product and sum in a single pass through the digits.
    ///
    /// # Approach
    /// 1. Repeatedly extract the last digit via `n % 10`
    /// 2. Multiply into the product accumulator, add to the sum accumulator
    /// 3. Return `product - sum`
    ///
    /// # Complexity
    /// - Time: O(d) where d is the number of digits
    /// - Space: O(1)
    pub fn subtract_product_and_sum(mut n: i32) -> i32 {
        let (mut product, mut sum) = (1, 0);
        while n != 0 {
            let digit = n % 10;
            n /= 10;
            product *= digit;
            sum += digit;
        }
        product - sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_234() {
        assert_eq!(Solution::subtract_product_and_sum(234), 15);
    }

    #[test]
    fn example_4421() {
        assert_eq!(Solution::subtract_product_and_sum(4421), 21);
    }

    #[test]
    fn single_digit() {
        assert_eq!(Solution::subtract_product_and_sum(5), -5 + 25);
        // product=5, sum=5, 5-5=0
        assert_eq!(Solution::subtract_product_and_sum(5), 0);
    }
}
