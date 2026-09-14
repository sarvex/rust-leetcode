impl Solution {
    /// Enumerate the 450 candidate even numbers and test multiset feasibility.
    ///
    /// # Intuition
    /// The question asks for *distinct numbers*, not permutations, so the
    /// cleanest way to avoid double counting is to iterate over the numbers
    /// themselves instead of over index triples. A valid number has a non-zero
    /// hundreds digit and an even ones digit, which leaves only
    /// `9 * 10 * 5 = 450` candidates regardless of the input. Each candidate is
    /// buildable exactly when the multiset of its three digits is contained in
    /// the multiset of `digits`.
    ///
    /// # Approach
    /// Tally the input into a fixed 10-entry table, then sweep every candidate
    /// `(hundreds, tens, ones)`. Containment is checked incrementally in that
    /// order: a position is satisfiable when its digit's available count
    /// strictly exceeds the number of earlier positions already holding that
    /// same digit. The check performed at a digit's *last* occurrence in the
    /// triple therefore encodes its full requirement, so three comparisons
    /// cover all ten digits. Counting the surviving candidates gives the answer
    /// with no deduplication structure needed.
    ///
    /// # Complexity
    /// - Time: O(n) — one pass to tally, then a bounded 450-candidate sweep
    /// - Space: O(1) — a single 10-entry count table
    pub fn total_numbers(digits: Vec<i32>) -> i32 {
        let available = digits.iter().fold([0u8; 10], |mut counts, &digit| {
            counts[digit as usize] += 1;
            counts
        });

        (1..10usize)
            .flat_map(|hundreds| {
                (0..10usize).flat_map(move |tens| {
                    (0..10usize)
                        .step_by(2)
                        .map(move |ones| (hundreds, tens, ones))
                })
            })
            .filter(|&(hundreds, tens, ones)| {
                available[hundreds] > 0
                    && available[tens] > u8::from(tens == hundreds)
                    && available[ones] > u8::from(ones == hundreds) + u8::from(ones == tens)
            })
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(Solution::total_numbers(vec![1, 2, 3, 4]), 12);
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::total_numbers(vec![0, 2, 2]), 2);
    }

    #[test]
    fn example_three() {
        assert_eq!(Solution::total_numbers(vec![6, 6, 6]), 1);
    }

    #[test]
    fn example_four() {
        assert_eq!(Solution::total_numbers(vec![1, 3, 5]), 0);
    }

    #[test]
    fn no_even_digit_available() {
        // Every digit is odd, so no number can end in an even digit.
        assert_eq!(Solution::total_numbers(vec![7, 9, 1, 3, 5]), 0);
    }

    #[test]
    fn leading_zero_blocked() {
        // Only 0 and 4 exist: 400 needs two zeros, so just 040 candidates remain
        // and those are rejected as leading zeros.
        assert_eq!(Solution::total_numbers(vec![0, 0, 4]), 1);
        // 004, 040 invalid; 400 valid.
        assert_eq!(Solution::total_numbers(vec![0, 0, 0]), 0);
    }

    #[test]
    fn duplicates_limit_reuse() {
        // 222 is impossible with a single 2; 202 and 220 use the pair of zeros.
        assert_eq!(Solution::total_numbers(vec![2, 0, 0]), 2);
    }

    #[test]
    fn all_even_max_length() {
        // 10 digits, every even value twice: brute-force cross-check below.
        let digits = vec![0, 0, 2, 2, 4, 4, 6, 6, 8, 8];
        assert_eq!(
            Solution::total_numbers(digits.clone()),
            brute_force(&digits)
        );
    }

    #[test]
    fn matches_brute_force_on_varied_inputs() {
        let cases = [
            vec![1, 2, 3],
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![9, 9, 9, 8, 8, 8, 0, 0, 1, 1],
            vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 4],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![4, 4, 4],
            vec![1, 0, 0],
        ];

        for digits in cases {
            assert_eq!(
                Solution::total_numbers(digits.clone()),
                brute_force(&digits),
                "mismatch for {digits:?}"
            );
        }
    }

    /// Reference implementation: try every ordered triple of distinct indices
    /// and record the distinct valid numbers in a 1000-slot presence table.
    fn brute_force(digits: &[i32]) -> i32 {
        let n = digits.len();
        let mut seen = [false; 1000];

        for i in 0..n {
            if digits[i] == 0 {
                continue;
            }
            for j in 0..n {
                if j == i {
                    continue;
                }
                for k in 0..n {
                    if k == i || k == j || digits[k] % 2 != 0 {
                        continue;
                    }
                    let value = digits[i] * 100 + digits[j] * 10 + digits[k];
                    seen[value as usize] = true;
                }
            }
        }

        seen.iter().filter(|present| **present).count() as i32
    }
}
