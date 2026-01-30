
impl Solution {
    /// Determines if change can be made for every lemonade customer.
    ///
    /// # Intuition
    /// Track $5 and $10 bill counts. For $20, prefer giving a $10 + $5
    /// over three $5 bills since $5 bills are more versatile.
    ///
    /// # Approach
    /// Process each bill greedily. For $10, use one $5. For $20, prefer
    /// $10 + $5, falling back to three $5 bills.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let (mut fives, mut tens) = (0i32, 0i32);
        for &bill in &bills {
            match bill {
                5 => fives += 1,
                10 => {
                    fives -= 1;
                    tens += 1;
                }
                _ => {
                    if tens > 0 {
                        tens -= 1;
                        fives -= 1;
                    } else {
                        fives -= 3;
                    }
                }
            }
            if fives < 0 {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_possible() {
        assert!(Solution::lemonade_change(vec![5, 5, 5, 10, 20]));
    }

    #[test]
    fn test_impossible() {
        assert!(!Solution::lemonade_change(vec![5, 5, 10, 10, 20]));
    }

    #[test]
    fn test_all_fives() {
        assert!(Solution::lemonade_change(vec![5, 5, 5, 5]));
    }

    #[test]
    fn test_first_not_five() {
        assert!(!Solution::lemonade_change(vec![10, 10]));
    }
}
