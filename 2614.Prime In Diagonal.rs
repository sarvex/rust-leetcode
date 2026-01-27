impl Solution {
    /// Find the largest prime on either diagonal of a square matrix.
    ///
    /// # Intuition
    /// Check both diagonals for primality and track the maximum prime found.
    ///
    /// # Approach
    /// 1. Iterate rows, checking elements at positions (i, i) and (i, n-1-i)
    /// 2. For each diagonal element, test primality via trial division
    /// 3. Track the maximum prime
    ///
    /// # Complexity
    /// - Time: O(n * √v) where v is the max element value
    /// - Space: O(1)
    pub fn diagonal_prime(nums: Vec<Vec<i32>>) -> i32 {
        let n = nums.len();

        nums.iter()
            .enumerate()
            .flat_map(|(i, row)| {
                let mut candidates = vec![row[i]];
                if n - 1 - i != i {
                    candidates.push(row[n - 1 - i]);
                }
                candidates
            })
            .filter(|&v| Self::is_prime(v))
            .max()
            .unwrap_or(0)
    }

    fn is_prime(n: i32) -> bool {
        if n < 2 {
            return false;
        }
        let upper = (n as f64).sqrt() as i32;
        (2..=upper).all(|i| n % i != 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::diagonal_prime(vec![vec![1, 2, 3], vec![5, 6, 7], vec![9, 10, 11],]),
            11
        );
    }

    #[test]
    fn test_no_prime() {
        assert_eq!(Solution::diagonal_prime(vec![vec![1, 4], vec![8, 1]]), 0);
    }

    #[test]
    fn test_single_element_prime() {
        assert_eq!(Solution::diagonal_prime(vec![vec![2]]), 2);
    }
}
