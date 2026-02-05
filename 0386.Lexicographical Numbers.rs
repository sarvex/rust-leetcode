impl Solution {
    /// Generates 1..n in lexicographical order using iterative DFS.
    ///
    /// # Intuition
    /// Lexicographical order corresponds to a pre-order traversal of a 10-ary
    /// trie where each node branches by appending digits 0-9.
    ///
    /// # Approach
    /// 1. Start at 1 and push onto the result.
    /// 2. Try to go deeper by multiplying by 10.
    /// 3. If that exceeds n, backtrack by dividing by 10 until we can increment.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) for the result vector
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut result = Vec::with_capacity(n as usize);
        let mut current = 1;
        for _ in 0..n {
            result.push(current);
            if current * 10 <= n {
                current *= 10;
            } else {
                while current % 10 == 9 || current + 1 > n {
                    current /= 10;
                }
                current += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thirteen() {
        assert_eq!(
            Solution::lexical_order(13),
            vec![1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9]
        );
    }

    #[test]
    fn test_two() {
        assert_eq!(Solution::lexical_order(2), vec![1, 2]);
    }

    #[test]
    fn test_one() {
        assert_eq!(Solution::lexical_order(1), vec![1]);
    }

    #[test]
    fn test_ten() {
        assert_eq!(
            Solution::lexical_order(10),
            vec![1, 10, 2, 3, 4, 5, 6, 7, 8, 9]
        );
    }

    #[test]
    fn test_twenty() {
        assert_eq!(
            Solution::lexical_order(20),
            vec![
                1, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 2, 20, 3, 4, 5, 6, 7, 8, 9
            ]
        );
    }

    #[test]
    fn test_hundred() {
        let result = Solution::lexical_order(100);
        assert_eq!(result.len(), 100);
        assert_eq!(result[0], 1);
        assert_eq!(result[1], 10);
        assert_eq!(result[11], 2);
        assert_eq!(result[99], 99);
    }

    #[test]
    fn test_nine() {
        assert_eq!(Solution::lexical_order(9), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
