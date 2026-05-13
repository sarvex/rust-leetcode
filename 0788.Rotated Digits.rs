impl Solution {
    /// Digit classification scan: count integers whose digits are all rotatable
    /// and that contain at least one digit whose rotation differs.
    ///
    /// # Intuition
    /// A number is "good" iff two conditions hold simultaneously on its digits:
    /// 1. Every digit is rotatable (one of `0,1,8,2,5,6,9`). Any `3,4,7`
    ///    invalidates the number.
    /// 2. At least one digit actually changes under rotation — i.e. is one of
    ///    `2,5,6,9`. Digits `0,1,8` rotate to themselves, so a number made only
    ///    of those equals its rotation and is therefore not "good".
    ///
    /// # Approach
    /// Iterate `x` from 1 to `n`. For each `x`, walk its decimal digits with
    /// integer division/modulo (no string conversion):
    /// - If any digit is `3`, `4`, or `7`, reject (invalid rotation).
    /// - If any digit is `2`, `5`, `6`, or `9`, mark the number as "changed".
    ///
    /// The number counts iff it is both valid and changed.
    ///
    /// Using arithmetic digit extraction avoids allocation and keeps the inner
    /// loop tight. For `n = 10_000` this is ~5·10^4 digit operations — trivial.
    ///
    /// # Complexity
    /// - Time: O(n · log10(n)) — constant ~5 digits per number for `n ≤ 10^4`.
    /// - Space: O(1)
    pub fn rotated_digits(n: i32) -> i32 {
        (1..=n).filter(|&x| Self::is_good(x)).count() as i32
    }

    /// Returns `true` iff `x` has only rotatable digits and at least one digit
    /// in `{2, 5, 6, 9}` (which forces the rotation to differ from `x`).
    fn is_good(mut x: i32) -> bool {
        let mut changed = false;
        while x > 0 {
            match x % 10 {
                3 | 4 | 7 => return false,
                2 | 5 | 6 | 9 => changed = true,
                _ => {}
            }
            x /= 10;
        }
        changed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_ten_yields_four() {
        assert_eq!(Solution::rotated_digits(10), 4);
    }

    #[test]
    fn example_one_yields_zero() {
        assert_eq!(Solution::rotated_digits(1), 0);
    }

    #[test]
    fn example_two_yields_one() {
        assert_eq!(Solution::rotated_digits(2), 1);
    }

    #[test]
    fn self_rotating_digits_are_excluded_up_to_eight() {
        assert_eq!(Solution::rotated_digits(8), 3);
    }

    #[test]
    fn matches_string_based_oracle_up_to_twenty() {
        let brute = (1..=20i32)
            .filter(|&x| {
                let s = x.to_string();
                s.chars().all(|c| "0125689".contains(c)) && s.chars().any(|c| "2569".contains(c))
            })
            .count() as i32;
        assert_eq!(Solution::rotated_digits(20), brute);
    }

    #[test]
    fn matches_arithmetic_oracle_at_constraint_max() {
        let n = 10_000;
        let expected = (1..=n)
            .filter(|&x| {
                let mut y = x;
                let mut changed = false;
                let mut valid = true;
                while y > 0 {
                    match y % 10 {
                        3 | 4 | 7 => {
                            valid = false;
                            break;
                        }
                        2 | 5 | 6 | 9 => changed = true,
                        _ => {}
                    }
                    y /= 10;
                }
                valid && changed
            })
            .count() as i32;
        assert_eq!(Solution::rotated_digits(n), expected);
    }
}
