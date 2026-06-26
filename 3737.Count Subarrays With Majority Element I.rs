impl Solution {
    /// Count subarrays where target is the majority element using prefix balance.
    ///
    /// # Intuition
    /// Map each element to +1 if it equals target, -1 otherwise. A subarray has
    /// target as majority iff its balance sum is positive, i.e., the prefix balance
    /// strictly increases from the start to the end of the subarray.
    ///
    /// # Approach
    /// Build a prefix balance array where balance[i] = sum of (+1/-1) for nums[0..i].
    /// For each pair (i, j) with i <= j, the subarray nums[i..=j] has positive balance
    /// iff balance[j+1] > balance[i]. Count all such pairs using a nested loop.
    ///
    /// # Complexity
    /// - Time: O(n²)
    /// - Space: O(n)
    pub fn count_majority_subarrays(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        // prefix[i] = balance after considering nums[0..i]
        let prefix: Vec<i32> = std::iter::once(0)
            .chain(nums.iter().map(|&x| if x == target { 1 } else { -1 }))
            .scan(0, |acc, x| {
                *acc += x;
                Some(*acc)
            })
            .collect();

        // Count pairs (i, j) where 0 <= i < j <= n and prefix[j] > prefix[i]
        // which corresponds to subarray nums[i..j-1] having target as majority
        let p = prefix.as_slice();
        (0..n)
            .flat_map(|i| (i + 1..=n).filter(move |&j| p[j] > p[i]))
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::count_majority_subarrays(vec![1, 2, 2, 3], 2), 5);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::count_majority_subarrays(vec![1, 1, 1, 1], 1), 10);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::count_majority_subarrays(vec![1, 2, 3], 4), 0);
    }

    #[test]
    fn test_single_element_match() {
        assert_eq!(Solution::count_majority_subarrays(vec![5], 5), 1);
    }

    #[test]
    fn test_single_element_no_match() {
        assert_eq!(Solution::count_majority_subarrays(vec![5], 3), 0);
    }

    #[test]
    fn test_alternating() {
        // [1,2,1,2]: target=1 appears in [1], [1], [1,2,1], [2,1,2] — wait, strict majority
        // subarrays: [1](idx0)=ok, [1](idx2)=ok, [1,2,1]=3 elems 2 ones=ok, [2,1,2]=no, [1,2,1,2]=no
        assert_eq!(Solution::count_majority_subarrays(vec![1, 2, 1, 2], 1), 3);
    }
}
