impl Solution {
    /// Smooths an image by averaging each cell's 3×3 neighborhood.
    ///
    /// # Intuition
    /// For each cell, sum all valid neighbors and divide by the count.
    ///
    /// # Approach
    /// 1. For each (i, j), iterate over the 3×3 window using saturating arithmetic.
    /// 2. Sum values and count cells, then floor-divide.
    ///
    /// # Complexity
    /// - Time: O(m × n) — constant 9 neighbors per cell
    /// - Space: O(m × n) for the result
    pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (img.len(), img[0].len());
        let mut result = vec![vec![0; n]; m];
        for i in 0..m {
            for j in 0..n {
                let mut sum = 0;
                let mut count = 0;
                for x in i.saturating_sub(1)..=(i + 1).min(m - 1) {
                    for y in j.saturating_sub(1)..=(j + 1).min(n - 1) {
                        sum += img[x][y];
                        count += 1;
                    }
                }
                result[i][j] = sum / count;
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
            Solution::image_smoother(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]),
            vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]
        );
    }

    #[test]
    fn test_larger() {
        assert_eq!(
            Solution::image_smoother(vec![
                vec![100, 200, 100],
                vec![200, 50, 200],
                vec![100, 200, 100]
            ]),
            vec![
                vec![137, 141, 137],
                vec![141, 138, 141],
                vec![137, 141, 137]
            ]
        );
    }
}
