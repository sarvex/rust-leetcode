use std::collections::{HashSet, VecDeque};

impl Solution {
    /// Finds the minimum mutations from start to end gene using BFS.
    ///
    /// # Intuition
    /// Each valid mutation changes exactly one character and must exist in the
    /// gene bank. BFS guarantees the shortest mutation path.
    ///
    /// # Approach
    /// 1. BFS from the start gene.
    /// 2. At each level, check all bank genes differing by exactly one character.
    /// 3. Track visited genes to avoid cycles.
    ///
    /// # Complexity
    /// - Time: O(B² · L) where B = bank size, L = gene length (8)
    /// - Space: O(B)
    pub fn min_mutation(start_gene: String, end_gene: String, bank: Vec<String>) -> i32 {
        let mut visited = HashSet::new();
        visited.insert(start_gene.clone());
        let mut queue = VecDeque::new();
        queue.push_back((start_gene, 0));

        while let Some((gene, depth)) = queue.pop_front() {
            if gene == end_gene {
                return depth;
            }
            let gene_bytes = gene.as_bytes();
            for next in &bank {
                if !visited.contains(next) {
                    let diff = next
                        .bytes()
                        .zip(gene_bytes.iter().copied())
                        .filter(|(a, b)| a != b)
                        .count();
                    if diff == 1 {
                        visited.insert(next.clone());
                        queue.push_back((next.clone(), depth + 1));
                    }
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_mutation() {
        assert_eq!(
            Solution::min_mutation(
                "AACCGGTT".to_string(),
                "AACCGGTA".to_string(),
                vec!["AACCGGTA".to_string()],
            ),
            1
        );
    }

    #[test]
    fn test_two_mutations() {
        assert_eq!(
            Solution::min_mutation(
                "AACCGGTT".to_string(),
                "AAACGGTA".to_string(),
                vec![
                    "AACCGGTA".to_string(),
                    "AACCGCTA".to_string(),
                    "AAACGGTA".to_string()
                ],
            ),
            2
        );
    }

    #[test]
    fn test_no_path() {
        assert_eq!(
            Solution::min_mutation(
                "AACCGGTT".to_string(),
                "AAACGGTA".to_string(),
                vec!["AACCGGTA".to_string()],
            ),
            -1
        );
    }
}
