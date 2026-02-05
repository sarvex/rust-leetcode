impl Solution {
    /// Performs flood fill via DFS from the starting pixel.
    ///
    /// # Intuition
    /// Starting from `(sr, sc)`, recursively paint all connected pixels that
    /// share the original color to the new color.
    ///
    /// # Approach
    /// If the starting pixel already matches the new color, return immediately.
    /// Otherwise DFS in four directions, painting each matching pixel.
    ///
    /// # Complexity
    /// - Time: O(m * n)
    /// - Space: O(m * n) recursion stack in worst case
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let (sr, sc) = (sr as usize, sc as usize);
        let original = image[sr][sc];
        if original == color {
            return image;
        }

        fn dfs(image: &mut Vec<Vec<i32>>, r: usize, c: usize, original: i32, color: i32) {
            image[r][c] = color;
            let (rows, cols) = (image.len(), image[0].len());
            [(!0usize, 0), (1, 0), (0, !0usize), (0, 1)]
                .into_iter()
                .map(|(dr, dc)| (r.wrapping_add(dr), c.wrapping_add(dc)))
                .filter(|&(nr, nc)| nr < rows && nc < cols && image[nr][nc] == original)
                .collect::<Vec<_>>()
                .into_iter()
                .for_each(|(nr, nc)| dfs(image, nr, nc, original, color));
        }

        dfs(&mut image, sr, sc, original, color);
        image
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_fill() {
        let image = vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]];
        let expected = vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]];
        assert_eq!(Solution::flood_fill(image, 1, 1, 2), expected);
    }

    #[test]
    fn test_same_color() {
        let image = vec![vec![0, 0, 0], vec![0, 0, 0]];
        let expected = vec![vec![0, 0, 0], vec![0, 0, 0]];
        assert_eq!(Solution::flood_fill(image, 0, 0, 0), expected);
    }

    #[test]
    fn test_single_pixel() {
        assert_eq!(Solution::flood_fill(vec![vec![1]], 0, 0, 2), vec![vec![2]]);
    }
}
