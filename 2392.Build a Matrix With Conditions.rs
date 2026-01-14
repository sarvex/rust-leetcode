impl Solution {
    /// Builds a k×k matrix satisfying row and column ordering conditions using topological sort
    ///
    /// # Intuition
    /// Row and column orderings are independent partial orderings solvable via topological sort.
    ///
    /// # Approach
    /// 1. Topological sort row conditions to get row positions
    /// 2. Topological sort column conditions to get column positions
    /// 3. Place each number at its (row, col) position
    ///
    /// # Complexity
    /// - Time: O(k + n + m)
    /// - Space: O(k + n + m)
    pub fn build_matrix(
        k: i32,
        row_conditions: Vec<Vec<i32>>,
        col_conditions: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        let k = k as usize;

        let row_pos = match Self::topo_sort(k, &row_conditions) {
            Some(p) => p,
            None => return vec![],
        };

        let col_pos = match Self::topo_sort(k, &col_conditions) {
            Some(p) => p,
            None => return vec![],
        };

        let mut matrix = vec![vec![0i32; k]; k];
        for num in 1..=k {
            matrix[row_pos[num]][col_pos[num]] = num as i32;
        }

        matrix
    }

    fn topo_sort(k: usize, conditions: &[Vec<i32>]) -> Option<Vec<usize>> {
        let n = k + 1;
        let mut degree = vec![0u32; n];
        let mut count = vec![0usize; n];

        for e in conditions {
            count[e[0] as usize] += 1;
        }

        let mut offset = vec![0usize; n + 1];
        for i in 1..=k {
            offset[i + 1] = offset[i] + count[i];
        }

        let mut edges = vec![0usize; conditions.len()];
        let mut pos = offset.clone();

        for e in conditions {
            let u = e[0] as usize;
            let v = e[1] as usize;
            edges[pos[u]] = v;
            pos[u] += 1;
            degree[v] += 1;
        }

        let mut queue = Vec::with_capacity(k);
        for i in 1..=k {
            if degree[i] == 0 {
                queue.push(i);
            }
        }

        let mut head = 0;
        while head < queue.len() {
            let u = queue[head];
            head += 1;
            for j in offset[u]..offset[u + 1] {
                let v = edges[j];
                degree[v] -= 1;
                if degree[v] == 0 {
                    queue.push(v);
                }
            }
        }

        if queue.len() != k {
            return None;
        }

        let mut result = vec![0usize; n];
        for (i, &node) in queue.iter().enumerate() {
            result[node] = i;
        }

        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn verify(matrix: &[Vec<i32>], k: i32, row_cond: &[Vec<i32>], col_cond: &[Vec<i32>]) -> bool {
        let k = k as usize;
        if matrix.len() != k {
            return false;
        }

        let mut pos = vec![(0, 0); k + 1];
        let mut seen = vec![false; k + 1];

        for (r, row) in matrix.iter().enumerate() {
            if row.len() != k {
                return false;
            }
            for (c, &val) in row.iter().enumerate() {
                if val > 0 {
                    let v = val as usize;
                    if v > k || seen[v] {
                        return false;
                    }
                    seen[v] = true;
                    pos[v] = (r, c);
                }
            }
        }

        (1..=k).all(|i| seen[i])
            && row_cond
                .iter()
                .all(|c| pos[c[0] as usize].0 < pos[c[1] as usize].0)
            && col_cond
                .iter()
                .all(|c| pos[c[0] as usize].1 < pos[c[1] as usize].1)
    }

    #[test]
    fn test_example_1() {
        let k = 3;
        let rc = vec![vec![1, 2], vec![3, 2]];
        let cc = vec![vec![2, 1], vec![3, 2]];
        let result = Solution::build_matrix(k, rc.clone(), cc.clone());
        assert!(verify(&result, k, &rc, &cc));
    }

    #[test]
    fn test_example_2_cycle() {
        let k = 3;
        let rc = vec![vec![1, 2], vec![2, 3], vec![3, 1], vec![2, 3]];
        let cc = vec![vec![2, 1]];
        let result = Solution::build_matrix(k, rc, cc);
        assert!(result.is_empty());
    }

    #[test]
    fn test_minimal() {
        let k = 2;
        let rc = vec![vec![1, 2]];
        let cc = vec![vec![1, 2]];
        let result = Solution::build_matrix(k, rc.clone(), cc.clone());
        assert!(verify(&result, k, &rc, &cc));
    }

    #[test]
    fn test_no_conditions() {
        let k = 3;
        let rc: Vec<Vec<i32>> = vec![];
        let cc: Vec<Vec<i32>> = vec![];
        let result = Solution::build_matrix(k, rc.clone(), cc.clone());
        assert!(verify(&result, k, &rc, &cc));
    }

    #[test]
    fn test_duplicates() {
        let k = 3;
        let rc = vec![vec![1, 2], vec![1, 2], vec![2, 3], vec![2, 3]];
        let cc = vec![vec![3, 1], vec![3, 1]];
        let result = Solution::build_matrix(k, rc.clone(), cc.clone());
        assert!(verify(&result, k, &rc, &cc));
    }
}
