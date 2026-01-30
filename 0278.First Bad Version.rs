// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

pub struct Solution;

impl Solution {
    /// Finds the first bad version using binary search.
    ///
    /// # Intuition
    /// Since all versions after the first bad one are also bad, the versions form
    /// a monotonic sequence: [good, good, ..., bad, bad, bad]. Binary search can
    /// efficiently find the transition point in O(log n) time.
    ///
    /// # Approach
    /// 1. Initialize search bounds: `left = 1`, `right = n`.
    /// 2. Compute `mid` avoiding integer overflow with `left + (right - left) / 2`.
    /// 3. If `mid` is bad, the first bad version is at `mid` or earlier; narrow to `[left, mid]`.
    /// 4. If `mid` is good, the first bad version is after `mid`; narrow to `[mid + 1, right]`.
    /// 5. When `left == right`, we've found the first bad version.
    ///
    /// # Complexity
    /// - Time: O(log n) - binary search halves the search space each iteration
    /// - Space: O(1) - only uses two pointer variables
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let (mut left, mut right) = (1, n);
        while left < right {
            let mid = left + (right - left) / 2;
            if self.isBadVersion(mid) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestSolution {
        bad_version: i32,
    }

    impl TestSolution {
        #[allow(non_snake_case)]
        fn isBadVersion(&self, version: i32) -> bool {
            version >= self.bad_version
        }

        fn first_bad_version(&self, n: i32) -> i32 {
            let (mut left, mut right) = (1, n);
            while left < right {
                let mid = left + (right - left) / 2;
                if self.isBadVersion(mid) {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }
            left
        }
    }

    #[test]
    fn test_first_bad_middle() {
        let solution = TestSolution { bad_version: 4 };
        assert_eq!(solution.first_bad_version(5), 4);
    }

    #[test]
    fn test_first_bad_at_start() {
        let solution = TestSolution { bad_version: 1 };
        assert_eq!(solution.first_bad_version(1), 1);
        assert_eq!(solution.first_bad_version(10), 1);
    }

    #[test]
    fn test_first_bad_at_end() {
        let solution = TestSolution { bad_version: 10 };
        assert_eq!(solution.first_bad_version(10), 10);
    }

    #[test]
    fn test_large_input() {
        let solution = TestSolution {
            bad_version: 1702766719,
        };
        assert_eq!(solution.first_bad_version(2126753390), 1702766719);
    }

    #[test]
    fn test_two_versions() {
        let solution = TestSolution { bad_version: 2 };
        assert_eq!(solution.first_bad_version(2), 2);
    }
}
