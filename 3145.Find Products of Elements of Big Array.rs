fn count_bit_set_up_to(max_value: u64, bit: u32) -> u128 {
    let period = 1u128 << (bit + 1);
    let half = 1u128 << bit;
    let total = max_value as u128 + 1;
    let full_cycles = total / period;
    let remainder = total % period;
    full_cycles * half + remainder.saturating_sub(half)
}

fn total_bits_up_to(max_value: u64) -> u128 {
    let mut total = 0u128;
    let mut bit = 0u32;
    let limit = max_value as u128;
    while (1u128 << bit) <= limit {
        total += count_bit_set_up_to(max_value, bit);
        bit += 1;
    }
    total
}

fn sum_bit_indices_up_to(max_value: u64) -> u128 {
    let mut total = 0u128;
    let mut bit = 0u32;
    let limit = max_value as u128;
    while (1u128 << bit) <= limit {
        let count = count_bit_set_up_to(max_value, bit);
        total += count * bit as u128;
        bit += 1;
    }
    total
}

fn sum_smallest_bit_indices(mut value: u64, mut take: u128) -> u128 {
    let mut total = 0u128;
    let mut bit = 0u32;
    while value > 0 && take > 0 {
        if value & 1 == 1 {
            total += bit as u128;
            take -= 1;
        }
        value >>= 1;
        bit += 1;
    }
    total
}

fn find_number_by_index(target: u128) -> u64 {
    if target == 0 {
        return 0;
    }
    let mut high = 1u64;
    while total_bits_up_to(high) < target {
        high = high.saturating_mul(2);
    }
    let mut low = 0u64;
    while low < high {
        let mid = low + (high - low + 1) / 2;
        if total_bits_up_to(mid) <= target {
            low = mid;
        } else {
            high = mid - 1;
        }
    }
    low
}

fn prefix_exponent_sum(elements: u128) -> u128 {
    if elements == 0 {
        return 0;
    }
    let last_number = find_number_by_index(elements);
    let used = total_bits_up_to(last_number);
    let mut total = sum_bit_indices_up_to(last_number);
    if used < elements {
        let remaining = elements - used;
        total += sum_smallest_bit_indices(last_number + 1, remaining);
    }
    total
}

fn mod_pow(mut base: u64, mut exp: u128, modu: u64) -> u64 {
    if modu == 1 {
        return 0;
    }
    base %= modu;
    let mut result = 1u64 % modu;
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result as u128 * base as u128 % modu as u128) as u64;
        }
        base = (base as u128 * base as u128 % modu as u128) as u64;
        exp >>= 1;
    }
    result
}

impl Solution {
    /// Prefix-sum bit indices to answer range products.
    ///
    /// # Intuition
    /// `big_nums` concatenates the powers of two from each integer's set bits, so every element is
    /// `2^k`. A product over a range equals `2^(sum of k)`, which means we only need the sum of bit
    /// indices in that range.
    ///
    /// # Approach
    /// Let `T(m)` be the total count of set bits in `[1..m]` and `F(m)` be the sum of their bit
    /// indices. For each bit `k`, its count up to `m` can be computed in O(1) using the repeating
    /// `0..1` pattern of length `2^(k+1)`. Summing over all `k` gives `T(m)` and `F(m)` in O(log m).
    /// For a prefix length `n`, binary-search the largest `m` with `T(m) <= n`, then add the lowest
    /// `n - T(m)` bit positions of `m + 1`. For a query `[l, r]`, the exponent is
    /// `S(r + 1) - S(l)`, and we compute `2^exponent mod mod`.
    ///
    /// # Complexity
    /// - Time: O(q log^2 M), where `M` is the largest index (<= 1e15)
    /// - Space: O(1)
    pub fn find_products_of_elements(queries: Vec<Vec<i64>>) -> Vec<i32> {
        queries
            .into_iter()
            .map(|query| {
                let from = query[0] as u128;
                let to = query[1] as u128;
                let modu = query[2] as u64;
                let exponent = prefix_exponent_sum(to + 1) - prefix_exponent_sum(from);
                mod_pow(2, exponent, modu) as i32
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_big_nums(limit: usize) -> Vec<u64> {
        let mut result = Vec::with_capacity(limit);
        let mut value = 1u64;
        while result.len() < limit {
            let mut bit = 0u32;
            let mut current = value;
            while current > 0 && result.len() < limit {
                if current & 1 == 1 {
                    result.push(1u64 << bit);
                }
                current >>= 1;
                bit += 1;
            }
            value += 1;
        }
        result
    }

    fn naive_query(nums: &[u64], left: usize, right: usize, modu: u64) -> u64 {
        if modu == 1 {
            return 0;
        }
        let mut result = 1u64 % modu;
        for &value in &nums[left..=right] {
            result = (result as u128 * (value % modu) as u128 % modu as u128) as u64;
        }
        result
    }

    #[test]
    fn test_example_1() {
        let queries = vec![vec![1, 3, 7]];
        assert_eq!(Solution::find_products_of_elements(queries), vec![4]);
    }

    #[test]
    fn test_example_2() {
        let queries = vec![vec![2, 5, 3], vec![7, 7, 4]];
        assert_eq!(Solution::find_products_of_elements(queries), vec![2, 2]);
    }

    #[test]
    fn test_small_ranges_against_naive() {
        let nums = build_big_nums(40);
        let cases = vec![(0, 0, 7), (1, 3, 7), (2, 5, 3), (7, 7, 4), (0, 9, 5), (10, 15, 11)];
        let queries: Vec<Vec<i64>> = cases
            .iter()
            .map(|&(left, right, modu)| vec![left as i64, right as i64, modu as i64])
            .collect();
        let expected: Vec<i32> = cases
            .iter()
            .map(|&(left, right, modu)| naive_query(&nums, left, right, modu) as i32)
            .collect();
        assert_eq!(Solution::find_products_of_elements(queries), expected);
    }

    #[test]
    fn test_mod_one() {
        let queries = vec![vec![0, 0, 1]];
        assert_eq!(Solution::find_products_of_elements(queries), vec![0]);
    }
}
