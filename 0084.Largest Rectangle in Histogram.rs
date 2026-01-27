impl Solution {
    /// Monotonic stack with boundary arrays for largest rectangle in histogram.
    ///
    /// # Intuition
    /// For each bar, the largest rectangle using that bar as the shortest
    /// extends from the nearest shorter bar on the left to the nearest
    /// shorter bar on the right. Monotonic stacks find these boundaries
    /// in linear time.
    ///
    /// # Approach
    /// Build two arrays: `left[i]` (index of nearest shorter bar to the left)
    /// and `right[i]` (nearest shorter to the right). Use an increasing
    /// monotonic stack scanning left-to-right for `left`, and right-to-left
    /// for `right`. The area for bar `i` is `heights[i] * (right[i] - left[i] - 1)`.
    ///
    /// # Complexity
    /// - Time: O(n) — each bar pushed and popped at most once per stack
    /// - Space: O(n) — boundary arrays and stack
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let n = heights.len();
        let mut left = vec![-1i32; n];
        let mut right = vec![n as i32; n];
        let mut stack: Vec<usize> = Vec::with_capacity(n);

        for i in 0..n {
            while let Some(&top) = stack.last() {
                if heights[top] >= heights[i] {
                    stack.pop();
                } else {
                    break;
                }
            }
            if let Some(&top) = stack.last() {
                left[i] = top as i32;
            }
            stack.push(i);
        }

        stack.clear();

        for i in (0..n).rev() {
            while let Some(&top) = stack.last() {
                if heights[top] >= heights[i] {
                    stack.pop();
                } else {
                    break;
                }
            }
            if let Some(&top) = stack.last() {
                right[i] = top as i32;
            }
            stack.push(i);
        }

        (0..n)
            .map(|i| heights[i] * (right[i] - left[i] - 1))
            .max()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_case() {
        assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
    }

    #[test]
    fn uniform_heights() {
        assert_eq!(Solution::largest_rectangle_area(vec![2, 4]), 4);
    }

    #[test]
    fn single_bar() {
        assert_eq!(Solution::largest_rectangle_area(vec![5]), 5);
    }

    #[test]
    fn ascending() {
        assert_eq!(Solution::largest_rectangle_area(vec![1, 2, 3, 4, 5]), 9);
    }

    #[test]
    fn descending() {
        assert_eq!(Solution::largest_rectangle_area(vec![5, 4, 3, 2, 1]), 9);
    }
}
