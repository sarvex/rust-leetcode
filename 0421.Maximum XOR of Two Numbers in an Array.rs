struct Trie {
    children: [Option<Box<Trie>>; 2],
}

impl Trie {
    fn new() -> Self {
        Self {
            children: [None, None],
        }
    }

    fn insert(&mut self, x: i32) {
        let mut node = self;
        for i in (0..=30).rev() {
            let bit = ((x >> i) & 1) as usize;
            node = node.children[bit].get_or_insert_with(|| Box::new(Trie::new()));
        }
    }

    fn max_xor(&self, x: i32) -> i32 {
        let mut node = self;
        let mut result = 0;
        for i in (0..=30).rev() {
            let bit = ((x >> i) & 1) as usize;
            let preferred = bit ^ 1;
            if let Some(child) = &node.children[preferred] {
                result |= 1 << i;
                node = child;
            } else {
                node = node.children[bit].as_ref().unwrap();
            }
        }
        result
    }
}

impl Solution {
    /// Finds the maximum XOR of any two numbers using a bitwise trie.
    ///
    /// # Intuition
    /// To maximize XOR, for each bit we greedily pick the opposite bit path in
    /// a binary trie. Inserting numbers one by one and querying on-the-fly
    /// captures all pairs.
    ///
    /// # Approach
    /// 1. Build a trie of 31-bit paths.
    /// 2. For each number, insert it then query for the maximum XOR against
    ///    all previously inserted numbers.
    /// 3. Track the global maximum.
    ///
    /// # Complexity
    /// - Time: O(31 · n) = O(n)
    /// - Space: O(31 · n) for trie nodes
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        let mut trie = Trie::new();
        nums.iter().fold(0, |best, &x| {
            trie.insert(x);
            best.max(trie.max_xor(x))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::find_maximum_xor(vec![3, 10, 5, 25, 2, 8]), 28);
    }

    #[test]
    fn test_all_same() {
        assert_eq!(Solution::find_maximum_xor(vec![7, 7, 7]), 0);
    }

    #[test]
    fn test_two_elements() {
        assert_eq!(
            Solution::find_maximum_xor(vec![14, 70, 53, 83, 49, 91, 36, 80, 92, 51, 66, 70]),
            127
        );
    }
}
