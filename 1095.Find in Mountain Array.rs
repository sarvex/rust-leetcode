// MountainArray's API interface.
pub struct MountainArray {
    arr: Vec<i32>,
}

impl MountainArray {
    pub fn get(&self, index: i32) -> i32 {
        self.arr[index as usize]
    }

    pub fn length(&self) -> i32 {
        self.arr.len() as i32
    }
}

impl Solution {
    /// Finds target in a mountain array using three binary searches.
    ///
    /// # Intuition
    /// First find the peak. Then binary search the ascending left half,
    /// and if not found, binary search the descending right half.
    ///
    /// # Approach
    /// 1. Binary search for the peak index.
    /// 2. Standard ascending binary search on `[0, peak]`.
    /// 3. Descending binary search on `[peak, n-1]` if not found.
    ///
    /// # Complexity
    /// - Time: O(log n)
    /// - Space: O(1)
    pub fn find_in_mountain_array(target: i32, mountain_arr: &MountainArray) -> i32 {
        let n = mountain_arr.length();
        let (mut lo, mut hi) = (0, n - 1);
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if mountain_arr.get(mid) > mountain_arr.get(mid + 1) {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        let peak = lo;

        let left = Self::binary_search(mountain_arr, 0, peak, 1, target);
        if left != -1 {
            return left;
        }
        Self::binary_search(mountain_arr, peak, n - 1, -1, target)
    }

    fn binary_search(m: &MountainArray, mut lo: i32, mut hi: i32, dir: i32, target: i32) -> i32 {
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if dir * m.get(mid) >= dir * target {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        if m.get(lo) == target {
            lo
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target_in_ascending_half() {
        let ma = MountainArray {
            arr: vec![1, 2, 3, 4, 5, 3, 1],
        };
        assert_eq!(Solution::find_in_mountain_array(3, &ma), 2);
    }

    #[test]
    fn target_at_peak() {
        let ma = MountainArray {
            arr: vec![1, 2, 5, 3, 1],
        };
        assert_eq!(Solution::find_in_mountain_array(5, &ma), 2);
    }

    #[test]
    fn target_only_in_descending_half() {
        let ma = MountainArray {
            arr: vec![1, 5, 4, 3, 2],
        };
        // 2 is not in [1,5], only in [5,4,3,2]
        assert_eq!(Solution::find_in_mountain_array(2, &ma), 4);
    }

    #[test]
    fn target_not_found() {
        let ma = MountainArray {
            arr: vec![1, 2, 3, 2, 1],
        };
        assert_eq!(Solution::find_in_mountain_array(4, &ma), -1);
    }

    #[test]
    fn target_prefers_leftmost() {
        let ma = MountainArray {
            arr: vec![0, 1, 2, 4, 2, 1],
        };
        assert_eq!(Solution::find_in_mountain_array(2, &ma), 2);
    }
}
