impl Solution {
    /// Finds intersections of two sorted interval lists using two pointers.
    ///
    /// # Intuition
    /// Two sorted interval lists can be merged by advancing the pointer
    /// with the earlier ending interval after checking for overlap.
    ///
    /// # Approach
    /// At each step, compute the overlap as `[max(starts), min(ends)]`.
    /// If valid, add to results. Advance the pointer whose interval ends first.
    ///
    /// # Complexity
    /// - Time: O(n + m)
    /// - Space: O(n + m) for the result
    pub fn interval_intersection(
        first_list: Vec<Vec<i32>>,
        second_list: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let (mut i, mut j) = (0, 0);
        while i < first_list.len() && j < second_list.len() {
            let start = first_list[i][0].max(second_list[j][0]);
            let end = first_list[i][1].min(second_list[j][1]);
            if start <= end {
                result.push(vec![start, end]);
            }
            if first_list[i][1] < second_list[j][1] {
                i += 1;
            } else {
                j += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::interval_intersection(
                vec![vec![0, 2], vec![5, 10], vec![13, 23], vec![24, 25]],
                vec![vec![1, 5], vec![8, 12], vec![15, 24], vec![25, 26]],
            ),
            vec![
                vec![1, 2],
                vec![5, 5],
                vec![8, 10],
                vec![15, 23],
                vec![24, 24],
                vec![25, 25]
            ]
        );
    }

    #[test]
    fn test_no_overlap() {
        assert!(Solution::interval_intersection(vec![vec![1, 3], vec![5, 9]], vec![],).is_empty());
    }
}
