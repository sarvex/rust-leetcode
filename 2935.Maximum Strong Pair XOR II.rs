struct TrieNode {
    children: [Option<usize>; 2],
    count: usize,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            children: [None; 2],
            count: 0,
        }
    }
}

struct Trie {
    nodes: Vec<TrieNode>,
    max_bit: u32,
}

impl Trie {
    fn new(max_bit: u32) -> Self {
        Self {
            nodes: vec![TrieNode::new()],
            max_bit,
        }
    }

    fn insert(&mut self, value: u32) {
        let mut idx = 0;
        self.nodes[idx].count += 1;
        for bit in (0..=self.max_bit).rev() {
            let b = ((value >> bit) & 1) as usize;
            let next = match self.nodes[idx].children[b] {
                Some(next) => next,
                None => {
                    let next = self.nodes.len();
                    self.nodes.push(TrieNode::new());
                    self.nodes[idx].children[b] = Some(next);
                    next
                }
            };
            idx = next;
            self.nodes[idx].count += 1;
        }
    }

    fn remove(&mut self, value: u32) {
        let mut idx = 0;
        self.nodes[idx].count -= 1;
        for bit in (0..=self.max_bit).rev() {
            let b = ((value >> bit) & 1) as usize;
            let next = self.nodes[idx].children[b].expect("value must exist in trie");
            idx = next;
            self.nodes[idx].count -= 1;
        }
    }

    fn max_xor(&self, value: u32) -> u32 {
        let mut idx = 0;
        let mut result = 0_u32;
        for bit in (0..=self.max_bit).rev() {
            let b = ((value >> bit) & 1) as usize;
            let preferred = b ^ 1;
            if let Some(next) = self.nodes[idx].children[preferred] {
                if self.nodes[next].count > 0 {
                    result |= 1_u32 << bit;
                    idx = next;
                    continue;
                }
            }
            let next = self.nodes[idx].children[b].expect("trie must contain at least one value");
            idx = next;
        }
        result
    }
}

impl Solution {
    /// Maximizes the XOR among all strong pairs using a sliding-window trie.
    ///
    /// # Intuition
    /// For a strong pair `(x, y)` with `x <= y`, the condition `|x - y| <= x` is equivalent to
    /// `y <= 2x`. After sorting, for each `y` the valid `x` values form a contiguous suffix where
    /// `nums[l] * 2 >= y`.
    ///
    /// # Approach
    /// - Sort `nums` in ascending order.
    /// - Maintain a window `[l, r]` that satisfies `nums[l] * 2 >= nums[r]`.
    /// - Store the window values in a bitwise trie with reference counts.
    /// - For each `r`, evict too-small values, insert `nums[r]`, and query the maximum XOR.
    ///
    /// # Complexity
    /// - Time: O(n · B), where `B` is the bit length (<= 20).
    /// - Space: O(n · B) for the trie nodes.
    pub fn maximum_strong_pair_xor(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();

        let max_value = *nums.last().expect("nums is non-empty") as u32;
        let max_bit = if max_value == 0 {
            0
        } else {
            31 - max_value.leading_zeros()
        };

        let mut trie = Trie::new(max_bit);
        let mut left = 0_usize;
        let mut best = 0_u32;

        for right in 0..nums.len() {
            let y = nums[right] as i64;
            while (nums[left] as i64) * 2 < y {
                trie.remove(nums[left] as u32);
                left += 1;
            }
            trie.insert(nums[right] as u32);
            best = best.max(trie.max_xor(nums[right] as u32));
        }

        best as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::maximum_strong_pair_xor(vec![1, 2, 3, 4, 5]), 7);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::maximum_strong_pair_xor(vec![10, 100]), 0);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(
            Solution::maximum_strong_pair_xor(vec![500, 520, 2500, 3000]),
            1020
        );
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::maximum_strong_pair_xor(vec![7]), 0);
    }

    #[test]
    fn test_two_elements_valid_pair() {
        assert_eq!(Solution::maximum_strong_pair_xor(vec![1, 2]), 3);
    }
}
