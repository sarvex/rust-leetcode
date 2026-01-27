impl Solution {
    /// Finds the kth largest element using quickselect partitioning.
    ///
    /// # Intuition
    /// Quickselect narrows the search to one partition at each step, achieving
    /// average O(n) time by only recursing into the relevant half.
    ///
    /// # Approach
    /// 1. Convert kth largest to index `n - k`.
    /// 2. Partition around the midpoint element using Hoare's scheme.
    /// 3. Recurse into the half containing the target index.
    ///
    /// # Complexity
    /// - Time: O(n) average, O(n^2) worst case
    /// - Space: O(log n) recursion stack
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        let len = nums.len();
        let target = len - k as usize;
        Self::quickselect(&mut nums, 0, len - 1, target)
    }

    fn quickselect(nums: &mut [i32], left: usize, right: usize, target: usize) -> i32 {
        if left == right {
            return nums[left];
        }
        let pivot = nums[(left + right) / 2];
        let (mut i, mut j) = (left as isize - 1, right as isize + 1);
        while i < j {
            i += 1;
            while nums[i as usize] < pivot {
                i += 1;
            }
            j -= 1;
            while nums[j as usize] > pivot {
                j -= 1;
            }
            if i < j {
                nums.swap(i as usize, j as usize);
            }
        }
        let j = j as usize;
        if target <= j {
            Self::quickselect(nums, left, j, target)
        } else {
            Self::quickselect(nums, j + 1, right, target)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_case() {
        assert_eq!(Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
    }

    #[test]
    fn with_duplicates() {
        assert_eq!(
            Solution::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4),
            4
        );
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::find_kth_largest(vec![1], 1), 1);
    }
}
