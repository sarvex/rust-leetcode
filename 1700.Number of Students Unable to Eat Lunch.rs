impl Solution {
    /// Counts remaining students using sandwich preference frequencies.
    ///
    /// # Intuition
    /// Students rotate in a queue, so the order doesn't matter — only the count
    /// of each preference matters. If a sandwich on top has no matching student,
    /// all remaining students are stuck.
    ///
    /// # Approach
    /// 1. Count how many students prefer each type (0 or 1).
    /// 2. Iterate through sandwiches in order.
    /// 3. If no student wants the current sandwich, return the sum of remaining students.
    /// 4. Otherwise decrement the matching count.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut count = [0i32; 2];
        students.iter().for_each(|&v| count[v as usize] += 1);
        for &v in &sandwiches {
            let idx = v as usize;
            if count[idx] == 0 {
                return count[idx ^ 1];
            }
            count[idx] -= 1;
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        assert_eq!(
            Solution::count_students(vec![1, 1, 0, 0], vec![0, 1, 0, 1]),
            0
        );
    }

    #[test]
    fn test_example_two() {
        assert_eq!(
            Solution::count_students(vec![1, 1, 1, 0, 0, 1], vec![1, 0, 0, 0, 1, 1]),
            3
        );
    }

    #[test]
    fn test_all_same_preference() {
        assert_eq!(Solution::count_students(vec![0, 0, 0], vec![0, 0, 0]), 0);
    }
}
