use std::collections::HashSet;

impl Solution {
    /// Returns the top k students ranked by feedback score, breaking ties by smaller ID.
    ///
    /// # Intuition
    /// Each report word matching a positive feedback word scores +3, negative scores -1.
    /// Build lookup sets for O(1) checks, compute scores, then sort.
    ///
    /// # Approach
    /// 1. Build HashSets for positive and negative feedback words
    /// 2. Score each student by scanning their report words
    /// 3. Sort by score descending, then by student ID ascending
    /// 4. Return the top k student IDs
    ///
    /// # Complexity
    /// - Time: O(n × m + n log n) where m is average report word count
    /// - Space: O(p + q + n) for feedback sets and score vector
    pub fn top_students(
        positive_feedback: Vec<String>,
        negative_feedback: Vec<String>,
        report: Vec<String>,
        student_id: Vec<i32>,
        k: i32,
    ) -> Vec<i32> {
        let positives: HashSet<&str> = positive_feedback.iter().map(|s| s.as_str()).collect();
        let negatives: HashSet<&str> = negative_feedback.iter().map(|s| s.as_str()).collect();

        let mut scored: Vec<(i32, i32)> = student_id
            .iter()
            .zip(report.iter())
            .map(|(&id, rep)| {
                let score = rep.split(' ').fold(0i32, |acc, word| {
                    if positives.contains(word) {
                        acc + 3
                    } else if negatives.contains(word) {
                        acc - 1
                    } else {
                        acc
                    }
                });
                (-score, id)
            })
            .collect();

        scored.sort_unstable();
        scored.iter().take(k as usize).map(|&(_, id)| id).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let pos = vec![
            "smart".to_string(),
            "brilliant".to_string(),
            "studious".to_string(),
        ];
        let neg = vec!["not".to_string()];
        let report = vec![
            "this student is studious".to_string(),
            "the student is smart".to_string(),
        ];
        let ids = vec![1, 2];
        assert_eq!(Solution::top_students(pos, neg, report, ids, 2), vec![1, 2]);
    }

    #[test]
    fn test_tie_breaking() {
        let pos = vec!["smart".to_string()];
        let neg = vec!["bad".to_string()];
        let report = vec!["smart".to_string(), "smart".to_string()];
        let ids = vec![5, 3];
        assert_eq!(Solution::top_students(pos, neg, report, ids, 1), vec![3]);
    }
}
