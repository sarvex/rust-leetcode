#[derive(Default)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

/// Singly linked list implementation using boxed nodes.
///
/// # Intuition
/// A linked list supports dynamic insertion and deletion at any index
/// by traversing to the target position and relinking pointers.
///
/// # Approach
/// Store the head as `Option<Box<ListNode>>`. Use a dummy node for
/// insertion and deletion to simplify edge cases at the head.
///
/// # Complexity
/// - Time: O(n) for get, add, and delete operations
/// - Space: O(n) for storing n elements
#[derive(Default)]
struct MyLinkedList {
    head: Option<Box<ListNode>>,
}

impl MyLinkedList {
    fn new() -> Self {
        Self::default()
    }

    fn get(&self, mut index: i32) -> i32 {
        let mut cur = match self.head.as_ref() {
            Some(node) => node,
            None => return -1,
        };
        while index > 0 {
            match cur.next {
                Some(ref next) => {
                    cur = next;
                    index -= 1;
                }
                None => return -1,
            }
        }
        cur.val
    }

    fn add_at_head(&mut self, val: i32) {
        self.head = Some(Box::new(ListNode {
            val,
            next: self.head.take(),
        }));
    }

    fn add_at_tail(&mut self, val: i32) {
        let new_node = Some(Box::new(ListNode { val, next: None }));
        match self.head.as_mut() {
            None => self.head = new_node,
            Some(mut cur) => {
                while let Some(ref mut next) = cur.next {
                    cur = next;
                }
                cur.next = new_node;
            }
        }
    }

    fn add_at_index(&mut self, mut index: i32, val: i32) {
        let mut dummy = Box::new(ListNode {
            val: 0,
            next: self.head.take(),
        });
        let mut cur = &mut dummy;
        while index > 0 {
            match cur.next.as_mut() {
                Some(next) => cur = next,
                None => return,
            }
            index -= 1;
        }
        cur.next = Some(Box::new(ListNode {
            val,
            next: cur.next.take(),
        }));
        self.head = dummy.next;
    }

    fn delete_at_index(&mut self, mut index: i32) {
        let mut dummy = Box::new(ListNode {
            val: 0,
            next: self.head.take(),
        });
        let mut cur = &mut dummy;
        while index > 0 {
            if let Some(ref mut next) = cur.next {
                cur = next;
            }
            index -= 1;
        }
        cur.next = cur.next.take().and_then(|n| n.next);
        self.head = dummy.next;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        let mut list = MyLinkedList::new();
        list.add_at_head(1);
        list.add_at_tail(3);
        list.add_at_index(1, 2);
        assert_eq!(list.get(1), 2);
        list.delete_at_index(1);
        assert_eq!(list.get(1), 3);
    }

    #[test]
    fn test_empty_list() {
        let list = MyLinkedList::new();
        assert_eq!(list.get(0), -1);
    }

    #[test]
    fn test_add_at_head_multiple() {
        let mut list = MyLinkedList::new();
        list.add_at_head(3);
        list.add_at_head(2);
        list.add_at_head(1);
        assert_eq!(list.get(0), 1);
        assert_eq!(list.get(1), 2);
        assert_eq!(list.get(2), 3);
    }

    #[test]
    fn test_add_at_tail_multiple() {
        let mut list = MyLinkedList::new();
        list.add_at_tail(1);
        list.add_at_tail(2);
        list.add_at_tail(3);
        assert_eq!(list.get(0), 1);
        assert_eq!(list.get(1), 2);
        assert_eq!(list.get(2), 3);
    }

    #[test]
    fn test_delete_head() {
        let mut list = MyLinkedList::new();
        list.add_at_head(1);
        list.add_at_head(0);
        list.delete_at_index(0);
        assert_eq!(list.get(0), 1);
    }

    #[test]
    fn test_out_of_bounds_get() {
        let mut list = MyLinkedList::new();
        list.add_at_head(1);
        assert_eq!(list.get(1), -1);
        assert_eq!(list.get(100), -1);
    }

    #[test]
    fn test_add_at_invalid_index() {
        let mut list = MyLinkedList::new();
        list.add_at_index(1, 10); // Should do nothing
        assert_eq!(list.get(0), -1);
    }
}
