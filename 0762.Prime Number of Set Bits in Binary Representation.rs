impl Solution {
    /// Count numbers in [left, right] with prime number of set bits.
    ///
    /// # Intuition
    /// For each number, count its set bits with `.count_ones()`. Only small counts
    /// (≤ 20 for n ≤ 10^6) are possible, so we use a bitset of primes up to 32.
    ///
    /// # Approach
    /// - Build a bitset where bit `i` is set iff `i` is prime (primes ≤ 32: 2,3,5,7,11,13,17,19,23,29,31).
    /// - Iterate [left, right], count set bits, add 1 to result if that count is prime.
    ///
    /// # Complexity
    /// - Time: O((right - left + 1) * O(count_ones)) = O(range), effectively O(10^4).
    /// - Space: O(1).
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        const PRIME_BITS: u32 = (1 << 2)
            | (1 << 3)
            | (1 << 5)
            | (1 << 7)
            | (1 << 11)
            | (1 << 13)
            | (1 << 17)
            | (1 << 19)
            | (1 << 23)
            | (1 << 29)
            | (1 << 31);

        (left..=right)
            .filter(|&n| ((PRIME_BITS >> n.count_ones()) & 1) != 0)
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::count_prime_set_bits(6, 10), 4);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::count_prime_set_bits(10, 15), 5);
    }

    #[test]
    fn test_single_prime_set_bits() {
        assert_eq!(Solution::count_prime_set_bits(2, 2), 0); // 2 -> 10 (1 bit, not prime)
        assert_eq!(Solution::count_prime_set_bits(3, 3), 1); // 3 -> 11 (2 bits, prime)
    }

    #[test]
    fn test_small_range() {
        assert_eq!(Solution::count_prime_set_bits(1, 1), 0); // 1 -> 1 (1 bit, not prime)
        assert_eq!(Solution::count_prime_set_bits(1, 5), 2); // 1,2,3,4,5 -> popcounts 1,1,2,1,2 -> primes at 3,5
    }
}
