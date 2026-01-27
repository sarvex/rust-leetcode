impl Solution {
    /// Position-based vote counting with multi-key sort.
    ///
    /// # Intuition
    /// Each team accumulates votes at each position. Sorting by the vote count
    /// at each position (descending), with alphabetical order as tiebreaker,
    /// determines the final ranking.
    ///
    /// # Approach
    /// 1. Build a 26 × m vote count matrix (team × position)
    /// 2. Tally each ballot's rankings into the matrix
    /// 3. Sort teams by their vote vectors (descending), breaking ties alphabetically
    ///
    /// # Complexity
    /// - Time: O(v·m + m·26·log 26) where v = number of votes, m = team count
    /// - Space: O(26·m) for the count matrix
    pub fn rank_teams(votes: Vec<String>) -> String {
        let m = votes[0].len();
        let mut count = vec![vec![0i32; m]; 26];

        for vote in &votes {
            for (i, b) in vote.bytes().enumerate() {
                count[(b - b'A') as usize][i] += 1;
            }
        }

        let mut teams: Vec<u8> = votes[0].bytes().collect();
        teams.sort_unstable_by(|&a, &b| {
            let ca = &count[(a - b'A') as usize];
            let cb = &count[(b - b'A') as usize];
            for k in 0..m {
                if ca[k] != cb[k] {
                    return cb[k].cmp(&ca[k]);
                }
            }
            a.cmp(&b)
        });

        String::from_utf8(teams).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_teams() {
        assert_eq!(
            Solution::rank_teams(vec![
                "ABC".to_string(),
                "ACB".to_string(),
                "ABC".to_string(),
                "ACB".to_string(),
                "ACB".to_string(),
            ]),
            "ACB"
        );
    }

    #[test]
    fn alphabetical_tiebreak() {
        assert_eq!(
            Solution::rank_teams(vec!["WXYZ".to_string(), "XYZW".to_string()]),
            "XWYZ"
        );
    }

    #[test]
    fn single_voter() {
        assert_eq!(
            Solution::rank_teams(vec!["ZMNAGUEDSJYLBOPHRQICWFXTVK".to_string()]),
            "ZMNAGUEDSJYLBOPHRQICWFXTVK"
        );
    }
}
