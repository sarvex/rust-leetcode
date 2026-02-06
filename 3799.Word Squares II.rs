impl Solution {
    /// Precompute first/last chars and sort once so iteration order yields sorted output.
    ///
    /// # Intuition
    /// Corner constraints (top[0]=left[0], top[3]=right[0], bottom[0]=left[3],
    /// bottom[3]=right[3]) allow early pruning. With words sorted, iterating
    /// (itop, ileft, iright, ibot) in index order produces squares in lexicographic
    /// order, so no final sort is needed.
    ///
    /// # Approach
    /// Sort words once. Precompute c0[i] = words[i][0] and c3[i] = words[i][3] for
    /// O(1) corner checks. Four nested loops over indices; skip when indices
    /// repeat or when first/last char constraints fail. Push valid squares in
    /// index order.
    ///
    /// # Complexity
    /// - Time: O(n log n + n^4) in the worst case; early continues reduce work.
    /// - Space: O(n) for c0/c3 plus O(S) for the result.
    pub fn word_squares(mut words: Vec<String>) -> Vec<Vec<String>> {
        words.sort_unstable();
        let n = words.len();

        let c0: Vec<u8> = words.iter().map(|w| w.as_bytes()[0]).collect();
        let c3: Vec<u8> = words.iter().map(|w| w.as_bytes()[3]).collect();

        let mut res = Vec::new();

        for itop in 0..n {
            let (t0, t3) = (c0[itop], c3[itop]);

            for ileft in 0..n {
                if c0[ileft] != t0 || ileft == itop {
                    continue;
                }
                let l3 = c3[ileft];

                for iright in 0..n {
                    if c0[iright] != t3 || iright == itop || iright == ileft {
                        continue;
                    }
                    let r3 = c3[iright];

                    for ibot in 0..n {
                        if c0[ibot] != l3 || c3[ibot] != r3 {
                            continue;
                        }
                        if ibot == itop || ibot == ileft || ibot == iright {
                            continue;
                        }
                        res.push(vec![
                            words[itop].clone(),
                            words[ileft].clone(),
                            words[iright].clone(),
                            words[ibot].clone(),
                        ]);
                    }
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let words = ["able", "area", "echo", "also"]
            .into_iter()
            .map(String::from)
            .collect();
        let got = Solution::word_squares(words);
        let want = [
            ["able", "area", "echo", "also"],
            ["area", "able", "also", "echo"],
        ]
        .map(|row| row.into_iter().map(String::from).collect::<Vec<_>>())
        .to_vec();
        assert_eq!(got, want);
    }

    #[test]
    fn test_example2() {
        let words = ["code", "cafe", "eden", "edge"]
            .into_iter()
            .map(String::from)
            .collect();
        let got = Solution::word_squares(words);
        assert!(got.is_empty());
    }

    #[test]
    fn test_sorted_output() {
        let words = ["area", "able", "also", "echo"]
            .into_iter()
            .map(String::from)
            .collect();
        let got = Solution::word_squares(words);
        let want = [
            ["able", "area", "echo", "also"],
            ["area", "able", "also", "echo"],
        ]
        .map(|row| row.into_iter().map(String::from).collect::<Vec<_>>())
        .to_vec();
        assert_eq!(got, want);
    }
}
