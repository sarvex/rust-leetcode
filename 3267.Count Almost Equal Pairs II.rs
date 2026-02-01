use std::collections::HashMap;

impl Solution {
    /// Count almost equal pairs by scanning numbers by digit length.
    ///
    /// # Intuition
    /// Swapping digits only permutes existing digits, so each number can reach a small, bounded set
    /// of values within two swaps. If two numbers differ in digit length, only the longer one can
    /// reach the shorter (by moving zeros to the front), so we can count pairs in length order.
    ///
    /// # Approach
    /// - Precompute digit vectors for each number and sort entries by digit length.
    /// - For each number in that order, enumerate all values reachable by up to two swaps of its
    ///   digits (deduplicated).
    /// - Count how many previously seen numbers match any reachable value.
    /// - Insert the current number into the frequency map.
    ///
    /// # Complexity
    /// - Time: O(n log n + n * L^4), with L <= 7.
    /// - Space: O(n + V), V <= 463 variants per number.
    pub fn count_pairs(nums: Vec<i32>) -> i32 {
        let mut entries: Vec<(Vec<u8>, i32)> = nums
            .into_iter()
            .map(|num| {
                let digits = digits(num);
                (digits, num)
            })
            .collect();
        entries.sort_by_key(|(digits, _)| digits.len());

        let mut seen: HashMap<i32, i32> = HashMap::with_capacity(entries.len());
        let mut total: i64 = 0;

        for (digits, num) in entries {
            let variants = compute_variants(&digits);
            for value in variants {
                if let Some(&count) = seen.get(&value) {
                    total += count as i64;
                }
            }
            *seen.entry(num).or_insert(0) += 1;
        }

        total as i32
    }
}

fn digits(mut num: i32) -> Vec<u8> {
    let mut rev_digits = Vec::new();
    while num > 0 {
        rev_digits.push((num % 10) as u8);
        num /= 10;
    }
    rev_digits.reverse();
    rev_digits
}

fn value_from_digits(digits: &[u8]) -> i32 {
    digits
        .iter()
        .fold(0_i32, |acc, &digit| acc * 10 + digit as i32)
}

fn compute_variants(digits: &[u8]) -> Vec<i32> {
    let len = digits.len();
    let original = value_from_digits(digits);
    if len < 2 {
        return vec![original];
    }

    let pair_count = len * (len - 1) / 2;
    let mut pairs = Vec::with_capacity(pair_count);
    for i in 0..len {
        for j in (i + 1)..len {
            pairs.push((i, j));
        }
    }

    let mut working = digits.to_vec();
    let mut variants = Vec::with_capacity(1 + pair_count + pair_count * pair_count);
    variants.push(original);

    for &(i, j) in &pairs {
        working.swap(i, j);
        variants.push(value_from_digits(&working));
        working.swap(i, j);
    }

    for &(i, j) in &pairs {
        working.swap(i, j);
        for &(k, l) in &pairs {
            working.swap(k, l);
            variants.push(value_from_digits(&working));
            working.swap(k, l);
        }
        working.swap(i, j);
    }

    variants.sort_unstable();
    variants.dedup();
    variants
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let nums = vec![1023, 2310, 2130, 213];
        assert_eq!(Solution::count_pairs(nums), 4);
    }

    #[test]
    fn example_two() {
        let nums = vec![1, 10, 100];
        assert_eq!(Solution::count_pairs(nums), 3);
    }

    #[test]
    fn duplicates_only() {
        let nums = vec![11, 11, 11];
        assert_eq!(Solution::count_pairs(nums), 3);
    }

    #[test]
    fn leading_zeros_match() {
        let nums = vec![10, 100, 1000];
        assert_eq!(Solution::count_pairs(nums), 3);
    }

    #[test]
    fn not_within_two_swaps() {
        let nums = vec![123456, 654321];
        assert_eq!(Solution::count_pairs(nums), 0);
    }
}
