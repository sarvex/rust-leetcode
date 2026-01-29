#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

/// Flattens a nested list structure using pre-computation via DFS.
///
/// # Intuition
/// Recursively flatten the entire structure at construction time into a
/// plain vector, then serve elements sequentially.
///
/// # Approach
/// 1. DFS through the nested list, collecting all integers into a vector.
/// 2. `next()` returns the current element and advances the index.
/// 3. `has_next()` checks if the index is within bounds.
///
/// # Complexity
/// - Time: O(n) construction, O(1) per next/has_next
/// - Space: O(n) for the flattened vector
struct NestedIterator {
    nums: Vec<i32>,
    index: usize,
}

impl NestedIterator {
    fn new(nested_list: Vec<NestedInteger>) -> Self {
        let mut nums = Vec::new();
        Self::flatten(&nested_list, &mut nums);
        Self { nums, index: 0 }
    }

    fn next(&mut self) -> i32 {
        let val = self.nums[self.index];
        self.index += 1;
        val
    }

    fn has_next(&self) -> bool {
        self.index < self.nums.len()
    }

    fn flatten(list: &[NestedInteger], result: &mut Vec<i32>) {
        for item in list {
            match item {
                NestedInteger::Int(x) => result.push(*x),
                NestedInteger::List(inner) => Self::flatten(inner, result),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nested_list_example1() {
        // [[1,1],2,[1,1]]
        let nested = vec![
            NestedInteger::List(vec![
                NestedInteger::Int(1),
                NestedInteger::Int(1),
            ]),
            NestedInteger::Int(2),
            NestedInteger::List(vec![
                NestedInteger::Int(1),
                NestedInteger::Int(1),
            ]),
        ];
        
        let mut iterator = NestedIterator::new(nested);
        
        assert!(iterator.has_next());
        assert_eq!(iterator.next(), 1);
        assert_eq!(iterator.next(), 1);
        assert_eq!(iterator.next(), 2);
        assert_eq!(iterator.next(), 1);
        assert_eq!(iterator.next(), 1);
        assert!(!iterator.has_next());
    }

    #[test]
    fn test_deeply_nested_list() {
        // [1,[4,[6]]]
        let nested = vec![
            NestedInteger::Int(1),
            NestedInteger::List(vec![
                NestedInteger::Int(4),
                NestedInteger::List(vec![
                    NestedInteger::Int(6),
                ]),
            ]),
        ];
        
        let mut iterator = NestedIterator::new(nested);
        
        assert_eq!(iterator.next(), 1);
        assert_eq!(iterator.next(), 4);
        assert_eq!(iterator.next(), 6);
        assert!(!iterator.has_next());
    }

    #[test]
    fn test_empty_nested_lists() {
        // [[],[]]
        let nested = vec![
            NestedInteger::List(vec![]),
            NestedInteger::List(vec![]),
        ];
        
        let iterator = NestedIterator::new(nested);
        assert!(!iterator.has_next());
    }

    #[test]
    fn test_single_integer() {
        // [0]
        let nested = vec![NestedInteger::Int(0)];
        
        let mut iterator = NestedIterator::new(nested);
        
        assert!(iterator.has_next());
        assert_eq!(iterator.next(), 0);
        assert!(!iterator.has_next());
    }

    #[test]
    fn test_complex_nested_structure() {
        // [[1,2],3,[4,[5,6,[7]]],8]
        let nested = vec![
            NestedInteger::List(vec![
                NestedInteger::Int(1),
                NestedInteger::Int(2),
            ]),
            NestedInteger::Int(3),
            NestedInteger::List(vec![
                NestedInteger::Int(4),
                NestedInteger::List(vec![
                    NestedInteger::Int(5),
                    NestedInteger::Int(6),
                    NestedInteger::List(vec![
                        NestedInteger::Int(7),
                    ]),
                ]),
            ]),
            NestedInteger::Int(8),
        ];
        
        let mut iterator = NestedIterator::new(nested);
        
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8];
        for val in expected {
            assert!(iterator.has_next());
            assert_eq!(iterator.next(), val);
        }
        assert!(!iterator.has_next());
    }

    #[test]
    fn test_negative_numbers() {
        // [[-1,-2],[-3]]
        let nested = vec![
            NestedInteger::List(vec![
                NestedInteger::Int(-1),
                NestedInteger::Int(-2),
            ]),
            NestedInteger::List(vec![
                NestedInteger::Int(-3),
            ]),
        ];
        
        let mut iterator = NestedIterator::new(nested);
        
        assert_eq!(iterator.next(), -1);
        assert_eq!(iterator.next(), -2);
        assert_eq!(iterator.next(), -3);
        assert!(!iterator.has_next());
    }
}
