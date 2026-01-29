pub struct Solution;

impl Solution {
    /// Duplicates each zero in-place, shifting elements right.
    ///
    /// # Intuition
    /// Two-pass approach: first count how many elements fit after duplication,
    /// then write backwards from that position.
    ///
    /// # Approach
    /// Pass 1: determine the effective end position after zero duplication.
    /// Pass 2: write elements from right to left, doubling zeros.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let n = arr.len();
        let (mut i, mut j) = (0, 0);
        while j < n {
            if arr[i] == 0 {
                j += 1;
            }
            j += 1;
            i += 1;
        }
        while i > 0 {
            i -= 1;
            j -= 1;
            if j < n {
                arr[j] = arr[i];
            }
            if arr[i] == 0 {
                j -= 1;
                if j < n {
                    arr[j] = 0;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut arr = vec![1, 0, 2, 3, 0, 4, 5, 0];
        Solution::duplicate_zeros(&mut arr);
        assert_eq!(arr, vec![1, 0, 0, 2, 3, 0, 0, 4]);
    }

    #[test]
    fn test_no_zeros() {
        let mut arr = vec![1, 2, 3];
        Solution::duplicate_zeros(&mut arr);
        assert_eq!(arr, vec![1, 2, 3]);
    }
}
