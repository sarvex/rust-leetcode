impl Solution {
    /// Finds the town judge using in-degree and out-degree counting.
    ///
    /// # Intuition
    /// The judge trusts nobody (out-degree 0) and is trusted by everyone
    /// else (in-degree n-1). Net degree = in - out = n-1 identifies the judge.
    ///
    /// # Approach
    /// Compute net trust degree for each person. The judge has net degree
    /// `n - 1`.
    ///
    /// # Complexity
    /// - Time: O(e + n) where e is trust pair count
    /// - Space: O(n) for the degree array
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut degree = vec![0i32; (n + 1) as usize];
        for t in &trust {
            degree[t[0] as usize] -= 1;
            degree[t[1] as usize] += 1;
        }
        (1..=n).find(|&i| degree[i as usize] == n - 1).unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::find_judge(2, vec![vec![1, 2]]), 2);
    }

    #[test]
    fn test_three_people() {
        assert_eq!(Solution::find_judge(3, vec![vec![1, 3], vec![2, 3]]), 3);
    }

    #[test]
    fn test_no_judge() {
        assert_eq!(
            Solution::find_judge(3, vec![vec![1, 3], vec![2, 3], vec![3, 1]]),
            -1
        );
    }

    #[test]
    fn test_single_person() {
        assert_eq!(Solution::find_judge(1, vec![]), 1);
    }
}
