#[derive(Clone, Copy)]
struct Candidate {
    idx: usize,
    value: i64,
}

impl Candidate {
    const fn new(idx: usize, value: i64) -> Self {
        Self { idx, value }
    }

    fn push_top3(top: &mut [Candidate; 3], idx: usize, value: i64) {
        if value > top[0].value {
            top[2] = top[1];
            top[1] = top[0];
            top[0] = Candidate::new(idx, value);
        } else if value > top[1].value {
            top[2] = top[1];
            top[1] = Candidate::new(idx, value);
        } else if value > top[2].value {
            top[2] = Candidate::new(idx, value);
        }
    }
}

impl Solution {
    /// Enumerate triples on the smaller dimension with top-3 candidates on the other.
    ///
    /// # Intuition
    /// For any fixed triple of columns, each column only needs its top-3 rows: among three
    /// columns, at most two rows are already used, so one top-3 row is always available
    /// without conflicts, and it is never worse than any lower-ranked row.
    ///
    /// # Approach
    /// If columns are the smaller dimension, compute the top-3 rows per column, then
    /// enumerate all triples of columns and all row choices from those candidates, keeping
    /// only distinct rows. If rows are smaller, do the symmetric operation on rows.
    ///
    /// # Complexity
    /// - Time: O(m * n + min(m, n)^3)
    /// - Space: O(m + n)
    pub fn maximum_value_sum(board: Vec<Vec<i32>>) -> i64 {
        let m = board.len();
        let n = board[0].len();
        let board: Vec<Vec<i64>> = board
            .into_iter()
            .map(|row| row.into_iter().map(|v| v as i64).collect())
            .collect();
        let invalid = Candidate::new(usize::MAX, i64::MIN);
        let mut best = i64::MIN;

        if n <= m {
            let mut col_candidates = vec![[invalid; 3]; n];
            for c in 0..n {
                let mut top = [invalid; 3];
                for r in 0..m {
                    Candidate::push_top3(&mut top, r, board[r][c]);
                }
                col_candidates[c] = top;
            }

            for c1 in 0..n - 2 {
                for c2 in c1 + 1..n - 1 {
                    for c3 in c2 + 1..n {
                        for cand1 in col_candidates[c1] {
                            let r1 = cand1.idx;
                            let v1 = cand1.value;
                            for cand2 in col_candidates[c2] {
                                let r2 = cand2.idx;
                                if r2 == r1 {
                                    continue;
                                }
                                let v2 = cand2.value;
                                for cand3 in col_candidates[c3] {
                                    let r3 = cand3.idx;
                                    if r3 == r1 || r3 == r2 {
                                        continue;
                                    }
                                    let sum = v1 + v2 + cand3.value;
                                    if sum > best {
                                        best = sum;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            let mut row_candidates = vec![[invalid; 3]; m];
            for r in 0..m {
                let mut top = [invalid; 3];
                for c in 0..n {
                    Candidate::push_top3(&mut top, c, board[r][c]);
                }
                row_candidates[r] = top;
            }

            for r1 in 0..m - 2 {
                for r2 in r1 + 1..m - 1 {
                    for r3 in r2 + 1..m {
                        for cand1 in row_candidates[r1] {
                            let c1 = cand1.idx;
                            let v1 = cand1.value;
                            for cand2 in row_candidates[r2] {
                                let c2 = cand2.idx;
                                if c2 == c1 {
                                    continue;
                                }
                                let v2 = cand2.value;
                                for cand3 in row_candidates[r3] {
                                    let c3 = cand3.idx;
                                    if c3 == c1 || c3 == c2 {
                                        continue;
                                    }
                                    let sum = v1 + v2 + cand3.value;
                                    if sum > best {
                                        best = sum;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        best
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let board = vec![vec![-3, 1, 1, 1], vec![-3, 1, -3, 1], vec![-3, 2, 1, 1]];
        assert_eq!(Solution::maximum_value_sum(board), 4);
    }

    #[test]
    fn test_example_2() {
        let board = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(Solution::maximum_value_sum(board), 15);
    }

    #[test]
    fn test_example_3() {
        let board = vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]];
        assert_eq!(Solution::maximum_value_sum(board), 3);
    }

    #[test]
    fn test_negative_values() {
        let board = vec![vec![-5, -1, -2], vec![-4, -9, -3], vec![-7, -6, -8]];
        assert_eq!(Solution::maximum_value_sum(board), -11);
    }
}
