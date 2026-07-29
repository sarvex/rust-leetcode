impl Solution {
    /// Finds the k-th lexicographically smallest palindromic permutation.
    ///
    /// # Intuition
    /// A palindrome is fully determined by its first half. Use the multinomial
    /// coefficient identity — n!/(f1!…fk!) = C(f1,f1)·C(f1+f2,f2)·… — to count
    /// permutations of the half exactly (capped at k) without ever dividing a
    /// pre-saturated numerator. GCD-reduced incremental binomial arithmetic
    /// keeps intermediate values exact up to the cap.
    ///
    /// # Approach
    /// 1. Count frequencies; validate palindrome parity; derive half_freq.
    /// 2. For each half position (left to right), try letters a–z in order.
    ///    Temporarily decrement, compute multinomial of the remainder, and
    ///    either commit the letter (if k_rem < count) or restore and subtract.
    /// 3. Build the full palindrome: half + optional middle char + half reversed.
    ///
    /// # Complexity
    /// - Time: O(26 × half_len × 26) — at most 26 distinct freqs, each binomial
    ///   step is O(min(f, rem-f)) ≤ O(26); total ≈ O(n) for practical inputs
    /// - Space: O(n) for result storage
    pub fn smallest_palindrome(s: String, k: i32) -> String {
        let n = s.len();
        let mut freq = [0i64; 26];
        s.bytes().for_each(|b| freq[(b - b'a') as usize] += 1);

        // Validate: at most one character may have odd frequency
        let odd_count = freq.iter().filter(|&&f| f % 2 == 1).count();
        if odd_count > 1 {
            return String::new();
        }

        let mid_char: Option<u8> = freq
            .iter()
            .enumerate()
            .find(|(_, f)| **f % 2 == 1)
            .map(|(i, _)| b'a' + i as u8);

        // Parity of n and presence of mid char must agree
        if (n % 2 == 0) != mid_char.is_none() {
            return String::new();
        }

        let mut half_freq = [0i64; 26];
        (0..26).for_each(|i| half_freq[i] = freq[i] / 2);
        let half_len: i64 = half_freq.iter().sum();

        if half_len == 0 {
            return if k == 1 {
                mid_char.map_or(String::new(), |c| String::from(c as char))
            } else {
                String::new()
            };
        }

        let cap = k as i64;

        // Counts permutations of a multiset whose frequencies are in `freq`.
        // Returns min(actual_count, cap+1) — we only need to distinguish
        // "less than cap" from "at least cap", so capping is safe.
        //
        // Uses the identity: multinomial(f1,…,fk) = prod C(f1+…+fi, fi)
        // Each binomial factor is computed with GCD-reduced incremental steps
        // so numerator and denominator stay exact (never artificially capped
        // before the division completes).
        let multinomial = |freq: &[i64; 26]| -> i64 {
            let mut result: i64 = 1;
            let mut running = 0i64;
            for &f in freq.iter() {
                if f == 0 {
                    continue;
                }
                running += f;
                // Multiply result by C(running, f), using the smaller side
                let k_side = f.min(running - f);
                for i in 0..k_side {
                    let num_factor = running - i;
                    let den_factor = i + 1;
                    // GCD-reduce to keep values small
                    let g1 = gcd(result, den_factor);
                    result /= g1;
                    let d = den_factor / g1;
                    let g2 = gcd(num_factor, d);
                    let n_red = num_factor / g2;
                    // d / g2 should be 1 here because C is always an integer
                    // but be safe: remaining denominator folded back
                    // (for C(n,k) the product is always exactly divisible)
                    result = result.saturating_mul(n_red);
                    if result > cap {
                        return cap + 1;
                    }
                }
            }
            result
        };

        let mut k_rem = k as i64 - 1; // convert to 0-indexed
        let mut result = Vec::with_capacity(half_len as usize);

        for _ in 0..half_len {
            let mut placed = false;
            for c in 0..26usize {
                if half_freq[c] == 0 {
                    continue;
                }
                half_freq[c] -= 1;
                let count = multinomial(&half_freq);
                if k_rem < count {
                    result.push(b'a' + c as u8);
                    placed = true;
                    break;
                }
                k_rem -= count;
                half_freq[c] += 1;
            }
            if !placed {
                return String::new(); // k exceeded total palindromes
            }
        }

        Self::build_palindrome(&result, mid_char)
    }

    fn build_palindrome(half: &[u8], mid: Option<u8>) -> String {
        let mut result = Vec::with_capacity(half.len() * 2 + usize::from(mid.is_some()));
        result.extend_from_slice(half);
        if let Some(m) = mid {
            result.push(m);
        }
        result.extend(half.iter().rev());
        String::from_utf8(result).unwrap()
    }
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::smallest_palindrome("abba".to_string(), 2), "baab");
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::smallest_palindrome("aa".to_string(), 2), "");
    }

    #[test]
    fn test_example_3() {
        assert_eq!(
            Solution::smallest_palindrome("bacab".to_string(), 1),
            "abcba"
        );
    }

    #[test]
    fn test_single_permutation() {
        assert_eq!(Solution::smallest_palindrome("aa".to_string(), 1), "aa");
    }

    #[test]
    fn test_first_permutation() {
        assert_eq!(Solution::smallest_palindrome("abba".to_string(), 1), "abba");
    }

    #[test]
    fn test_single_character() {
        assert_eq!(Solution::smallest_palindrome("o".to_string(), 1), "o");
    }

    #[test]
    fn test_multiple_same_chars() {
        assert_eq!(Solution::smallest_palindrome("aabb".to_string(), 1), "abba");
        assert_eq!(Solution::smallest_palindrome("aabb".to_string(), 2), "baab");
    }

    #[test]
    fn test_failing_case() {
        // half = {d:1,g:1,h:2}, 12 total perms, k=5 → half="ghdh"
        assert_eq!(
            Solution::smallest_palindrome("ghdhhdhg".to_string(), 5),
            "ghdhhdhg"
        );
    }

    #[test]
    fn test_k_out_of_range() {
        // "abba" has only 2 palindromic perms: abba, baab
        assert_eq!(Solution::smallest_palindrome("abba".to_string(), 3), "");
    }
}
