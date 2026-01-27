impl Solution {
    /// Monotonic stack for next-smaller-or-equal element discount.
    ///
    /// # Intuition
    /// Scanning right-to-left with a monotonic stack maintains the nearest
    /// future element ≤ current price. The stack top gives the discount.
    ///
    /// # Approach
    /// 1. Iterate prices from right to left
    /// 2. Pop stack elements greater than current price
    /// 3. Subtract stack top (if any) from current price
    /// 4. Push original price onto stack
    ///
    /// # Complexity
    /// - Time: O(n) amortized — each element pushed and popped at most once
    /// - Space: O(n) for the stack
    pub fn final_prices(mut prices: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<i32> = Vec::new();
        for i in (0..prices.len()).rev() {
            let original = prices[i];
            while stack.last().is_some_and(|&top| original < top) {
                stack.pop();
            }
            if let Some(&discount) = stack.last() {
                prices[i] -= discount;
            }
            stack.push(original);
        }
        prices
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_discount() {
        assert_eq!(
            Solution::final_prices(vec![8, 4, 6, 2, 3]),
            vec![4, 2, 4, 2, 3]
        );
    }

    #[test]
    fn decreasing_prices() {
        assert_eq!(
            Solution::final_prices(vec![1, 2, 3, 4, 5]),
            vec![1, 2, 3, 4, 5]
        );
    }

    #[test]
    fn all_same() {
        assert_eq!(Solution::final_prices(vec![10, 10, 10]), vec![0, 0, 10]);
    }
}
