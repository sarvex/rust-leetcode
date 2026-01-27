impl Solution {
    /// Enumerate all simplified fractions with denominator up to n.
    ///
    /// # Intuition
    /// A fraction i/j is in simplest form when gcd(i, j) == 1. Enumerate all
    /// pairs with numerator < denominator and filter by coprimality.
    ///
    /// # Approach
    /// 1. For each denominator j from 2 to n
    /// 2. For each numerator i from 1 to j-1
    /// 3. Include i/j if gcd(i, j) == 1
    ///
    /// # Complexity
    /// - Time: O(n² log n) accounting for GCD computations
    /// - Space: O(n²) worst case for the result
    pub fn simplified_fractions(n: i32) -> Vec<String> {
        fn gcd(a: i32, b: i32) -> i32 {
            if b == 0 {
                a
            } else {
                gcd(b, a % b)
            }
        }

        (2..=n)
            .flat_map(|j| {
                (1..j)
                    .filter(move |&i| gcd(i, j) == 1)
                    .map(move |i| format!("{i}/{j}"))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_equals_2() {
        assert_eq!(Solution::simplified_fractions(2), vec!["1/2"]);
    }

    #[test]
    fn n_equals_4() {
        let result = Solution::simplified_fractions(4);
        assert!(result.contains(&"1/2".to_string()));
        assert!(result.contains(&"1/3".to_string()));
        assert!(result.contains(&"2/3".to_string()));
        assert!(result.contains(&"1/4".to_string()));
        assert!(result.contains(&"3/4".to_string()));
        assert_eq!(result.len(), 5);
    }

    #[test]
    fn n_equals_1() {
        assert!(Solution::simplified_fractions(1).is_empty());
    }
}
