impl Solution {
    /// Track the two largest elements in a single pass.
    ///
    /// # Intuition
    /// The maximum product `(a-1)(b-1)` is achieved with the two largest
    /// elements. A single pass tracking the top two values avoids sorting.
    ///
    /// # Approach
    /// 1. Maintain the largest and second largest values
    /// 2. For each element, update both if necessary
    /// 3. Return `(max1 - 1) * (max2 - 1)`
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let (mut max1, mut max2) = (0, 0);
        for &num in &nums {
            if num > max1 {
                max2 = max1;
                max1 = num;
            } else if num > max2 {
                max2 = num;
            }
        }
        (max1 - 1) * (max2 - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_case() {
        assert_eq!(Solution::max_product(vec![3, 4, 5, 2]), 12);
    }

    #[test]
    fn with_ones() {
        assert_eq!(Solution::max_product(vec![1, 5, 4, 5]), 16);
    }

    #[test]
    fn small_array() {
        assert_eq!(Solution::max_product(vec![3, 7]), 12);
    }
}
