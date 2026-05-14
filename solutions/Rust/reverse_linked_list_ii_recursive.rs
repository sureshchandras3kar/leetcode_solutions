use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

/**
 * Reverse a portion of a linked list from position left to right using recursion.
 *
 * Time Complexity: O(n) - traverse to left, then reverse to right
 * Space Complexity: O(n) - recursion call stack
 *
 * Strategy:
 * 1. If left == 1, call reverseN() to reverse the first 'right' nodes
 * 2. Otherwise, recursively process the next node with adjusted positions
 */

/**
 * Helper function to reverse the first n nodes of a linked list.
 *
 * Returns (new_head, tail_node) where new_head is the head after reversal
 * and tail_node should be connected to the successor.
 */
fn reverse_n(
    head: Option<Rc<RefCell<ListNode>>>,
    n: i32,
) -> (Option<Rc<RefCell<ListNode>>>, Option<Rc<RefCell<ListNode>>>) {
    match head {
        Some(node) => {
            if n == 1 {
                // Base case: return the head and its next as successor
                let successor = node.borrow_mut().next.take();
                (Some(node), successor)
            } else {
                // Recursively reverse the rest
                let next_node = node.borrow_mut().next.take();
                let (new_head, successor) = reverse_n(next_node, n - 1);

                // Reconstruct the reversal
                if let Some(ref head_ref) = new_head {
                    // Find the current node's position in the reversed chain
                    // We need to connect it properly
                    let mut curr = head_ref.clone();
                    for _ in 1..n - 1 {
                        if let Some(ref next_ref) = curr.borrow().next {
                            curr = next_ref.clone();
                        }
                    }

                    // curr is now the tail of reversed section up to n-1
                    curr.borrow_mut().next = Some(node.clone());
                }

                node.borrow_mut().next = successor;
                (new_head, Some(node))
            }
        }
        None => (None, None),
    }
}

pub fn reverse_between(
    head: Option<Rc<RefCell<ListNode>>>,
    left: i32,
    right: i32,
) -> Option<Rc<RefCell<ListNode>>> {
    if left == right {
        return head;
    }

    if left == 1 {
        // Reverse the first 'right' nodes
        let (new_head, _) = reverse_n(head, right);
        new_head
    } else {
        // Recursively process the rest of the list
        match head {
            Some(node) => {
                let next = node.borrow_mut().next.take();
                let reversed_next = reverse_between(next, left - 1, right - 1);
                node.borrow_mut().next = reversed_next;
                Some(node)
            }
            None => None,
        }
    }
}

// Alternative cleaner recursive implementation using manual tracking
pub fn reverse_between_v2(
    head: Option<Rc<RefCell<ListNode>>>,
    left: i32,
    right: i32,
) -> Option<Rc<RefCell<ListNode>>> {
    fn reverse_range(
        node: Option<Rc<RefCell<ListNode>>>,
        left: i32,
        right: i32,
        pos: i32,
    ) -> Option<Rc<RefCell<ListNode>>> {
        match node {
            Some(mut n) => {
                let node_ref = Rc::get_mut(&mut n)?;
                if left == 1 {
                    // Start reversing
                    let (reversed, _) = reverse_first_n(Some(n), right);
                    reversed
                } else {
                    // Continue traversing
                    let next = std::mem::take(&mut node_ref.next);
                    node_ref.next = reverse_range(next, left - 1, right - 1, pos + 1);
                    Some(n)
                }
            }
            None => None,
        }
    }

    fn reverse_first_n(
        head: Option<Rc<RefCell<ListNode>>>,
        n: i32,
    ) -> (Option<Rc<RefCell<ListNode>>>, Option<Rc<RefCell<ListNode>>>) {
        match head {
            Some(mut node) => {
                let node_ref = Rc::get_mut(&mut node).unwrap();
                if n == 1 {
                    let succ = std::mem::take(&mut node_ref.next);
                    (Some(node), succ)
                } else {
                    let next = std::mem::take(&mut node_ref.next);
                    let (new_head, succ) = reverse_first_n(next, n - 1);

                    if let Some(ref nh) = new_head {
                        let mut curr = nh.clone();
                        for _ in 1..n - 1 {
                            if let Some(ref next_ref) = curr.borrow().next {
                                curr = next_ref.clone();
                            }
                        }
                        curr.borrow_mut().next = Some(node.clone());
                    }

                    node_ref.next = succ;
                    (new_head, Some(node))
                }
            }
            None => (None, None),
        }
    }

    reverse_range(head, left, right, 1)
}

// Helper function to create a linked list from a vector
fn create_list(values: Vec<i32>) -> Option<Rc<RefCell<ListNode>>> {
    if values.is_empty() {
        return None;
    }

    let mut head = Rc::new(RefCell::new(ListNode::new(values[0])));
    let mut current = head.clone();

    for val in &values[1..] {
        let new_node = Rc::new(RefCell::new(ListNode::new(*val)));
        current.borrow_mut().next = Some(new_node.clone());
        current = new_node;
    }

    Some(head)
}

// Helper function to convert linked list to vector
fn list_to_vec(head: Option<Rc<RefCell<ListNode>>>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut current = head;

    while let Some(node) = current {
        result.push(node.borrow().val);
        current = node.borrow_mut().next.take();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_middle_portion() {
        let head = create_list(vec![1, 2, 3, 4, 5]);
        let result = reverse_between(head, 2, 4);
        assert_eq!(list_to_vec(result), vec![1, 4, 3, 2, 5]);
    }

    #[test]
    fn test_single_node() {
        let head = create_list(vec![5]);
        let result = reverse_between(head, 1, 1);
        assert_eq!(list_to_vec(result), vec![5]);
    }

    #[test]
    fn test_reverse_entire_list() {
        let head = create_list(vec![1, 2]);
        let result = reverse_between(head, 1, 2);
        assert_eq!(list_to_vec(result), vec![2, 1]);
    }

    #[test]
    fn test_reverse_from_start() {
        let head = create_list(vec![1, 2, 3, 4, 5]);
        let result = reverse_between(head, 1, 3);
        assert_eq!(list_to_vec(result), vec![3, 2, 1, 4, 5]);
    }

    #[test]
    fn test_reverse_at_end() {
        let head = create_list(vec![1, 2, 3, 4, 5]);
        let result = reverse_between(head, 3, 5);
        assert_eq!(list_to_vec(result), vec![1, 2, 5, 4, 3]);
    }
}

fn main() {
    // Test 1: Reverse middle portion
    let head1 = create_list(vec![1, 2, 3, 4, 5]);
    let result1 = reverse_between(head1, 2, 4);
    println!("Test 1: {:?}", list_to_vec(result1));  // [1, 4, 3, 2, 5]

    // Test 2: Single node (no reversal)
    let head2 = create_list(vec![5]);
    let result2 = reverse_between(head2, 1, 1);
    println!("Test 2: {:?}", list_to_vec(result2));  // [5]

    // Test 3: Reverse entire list
    let head3 = create_list(vec![1, 2]);
    let result3 = reverse_between(head3, 1, 2);
    println!("Test 3: {:?}", list_to_vec(result3));  // [2, 1]

    // Test 4: Reverse from start
    let head4 = create_list(vec![1, 2, 3, 4, 5]);
    let result4 = reverse_between(head4, 1, 3);
    println!("Test 4: {:?}", list_to_vec(result4));  // [3, 2, 1, 4, 5]

    // Test 5: Reverse at end
    let head5 = create_list(vec![1, 2, 3, 4, 5]);
    let result5 = reverse_between(head5, 3, 5);
    println!("Test 5: {:?}", list_to_vec(result5));  // [1, 2, 5, 4, 3]
}
