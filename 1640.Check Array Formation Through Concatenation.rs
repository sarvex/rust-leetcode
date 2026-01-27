use std::collections::HashMap;

impl Solution {
    /// Hash map lookup matching pieces to array positions.
    ///
    /// # Intuition
    /// Map each piece's first element to its index. Walk through arr,
    /// looking up the matching piece by first element, then verify the
    /// entire piece matches consecutively.
    ///
    /// # Approach
    /// 1. Build map from first element of each piece to piece index
    /// 2. Walk arr, consuming full pieces at each position
    /// 3. Return false on any mismatch
    ///
    /// # Complexity
    /// - Time: O(n) where n = arr.len()
    /// - Space: O(k) where k = pieces.len()
    pub fn can_form_array(arr: Vec<i32>, pieces: Vec<Vec<i32>>) -> bool {
        let map: HashMap<i32, usize> = pieces.iter().enumerate().map(|(i, p)| (p[0], i)).collect();

        let mut i = 0;
        while i < arr.len() {
            match map.get(&arr[i]) {
                None => return false,
                Some(&idx) => {
                    for &val in &pieces[idx] {
                        if arr[i] != val {
                            return false;
                        }
                        i += 1;
                    }
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_form() {
        assert!(Solution::can_form_array(vec![85], vec![vec![85]],));
    }

    #[test]
    fn multi_piece() {
        assert!(Solution::can_form_array(
            vec![15, 88],
            vec![vec![88], vec![15]],
        ));
    }

    #[test]
    fn cannot_form() {
        assert!(!Solution::can_form_array(
            vec![49, 18, 16],
            vec![vec![16, 18, 49]],
        ));
    }
}
