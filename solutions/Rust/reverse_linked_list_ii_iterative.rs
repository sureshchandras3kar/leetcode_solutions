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
 * Reverse a portion of a linked list from position left to right (1-indexed).
 *
 * Time Complexity: O(n) - single pass through the list
 * Space Complexity: O(1) - only pointer variables
 *
 * Strategy:
 * 1. Create a dummy node to handle edge case of reversing from head
 * 2. Advance prev pointer to position left-1
 * 3. Perform (right - left) reversals by moving nodes to the front of the sublist
 * 4. Return dummy.next
 */
pub fn reverse_between(
    head: Option<Rc<RefCell<ListNode>>>,
    left: i32,
    right: i32,
) -> Option<Rc<RefCell<ListNode>>> {
    if left == right {
        return head;
    }

    // Create dummy node pointing to head
    let mut dummy = Rc::new(RefCell::new(ListNode::new(0)));
    dummy.borrow_mut().next = head;

    let mut prev = dummy.clone();

    // Advance prev to position left-1
    for _ in 0..left - 1 {
        let next_node = prev.borrow_mut().next.take();
        if let Some(next) = next_node {
            prev = next;
        }
    }

    // curr is the first node to reverse (at position left)
    if let Some(curr_node) = prev.borrow_mut().next.take() {
        let mut curr = curr_node;

        // Perform (right - left) reversals
        for _ in 0..right - left {
            // Get the node we want to move to the front
            if let Some(next_temp) = curr.borrow_mut().next.take() {
                // curr.next = next_temp.next
                curr.borrow_mut().next = next_temp.borrow_mut().next.take();

                // Move next_temp to the front of the sublist
                next_temp.borrow_mut().next = prev.borrow_mut().next.take();
                prev.borrow_mut().next = Some(next_temp);
            }
        }

        // Restore curr to prev.next
        prev.borrow_mut().next = Some(curr);
    }

    dummy.borrow_mut().next.take()
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
