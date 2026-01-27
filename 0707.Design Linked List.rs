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
