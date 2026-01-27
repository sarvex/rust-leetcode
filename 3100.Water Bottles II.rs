impl Solution {
    /// Maximum water bottles drunk with increasing exchange cost.
    ///
    /// # Intuition
    /// Greedily drink all full bottles, then exchange empties for one full bottle
    /// each time the exchange cost is met. The cost increases by one after each
    /// exchange, so we keep exchanging until we cannot afford it.
    ///
    /// # Approach
    /// 1. Start with ans = num_bottles (drink all initially).
    /// 2. While empty bottles >= exchange cost, pay the cost, get one bottle,
    ///    increment the exchange cost and the answer.
    /// 3. Return total bottles drunk.
    ///
    /// # Complexity
    /// - Time: O(sqrt(num_bottles)) — exchange cost grows linearly
    /// - Space: O(1)
    pub fn max_bottles_drunk(mut num_bottles: i32, mut num_exchange: i32) -> i32 {
        let mut ans = num_bottles;

        while num_bottles >= num_exchange {
            num_bottles -= num_exchange;
            num_exchange += 1;
            num_bottles += 1;
            ans += 1;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_exchange() {
        assert_eq!(Solution::max_bottles_drunk(13, 6), 15);
    }

    #[test]
    fn high_exchange_cost() {
        assert_eq!(Solution::max_bottles_drunk(10, 3), 13);
    }
}
