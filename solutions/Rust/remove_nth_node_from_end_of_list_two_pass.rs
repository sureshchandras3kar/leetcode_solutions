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

/*
Remove the nth node from the end of a linked list using two passes.

Approach:
1. First pass: count the total number of nodes
2. Second pass: find and skip the nth node from the end

Time: O(L) + O(L-n) = O(L)
Space: O(1)
*/
pub fn remove_nth_node_two_pass(head: Option<Rc<RefCell<ListNode>>>, n: i32) -> Option<Rc<RefCell<ListNode>>> {
    // First pass: count the total nodes
    let mut length = 0;
    let mut current = head.as_ref();
    while let Some(node) = current {
        length += 1;
        current = node.borrow().next.as_ref();
    }

    // Edge case: removing the head node
    if length == n {
        return head.and_then(|node| node.borrow_mut().next.take());
    }

    // Second pass: find the node before the one to remove
    if let Some(head_node) = head {
        let mut current = Some(head_node.clone());
        for _ in 0..length - n - 1 {
            if let Some(node) = current {
                current = node.borrow_mut().next.take();
            }
        }

        if let Some(node) = current {
            let mut borrowed = node.borrow_mut();
            if let Some(next) = borrowed.next.take() {
                borrowed.next = next.borrow_mut().next.take();
            }
        }

        return Some(head_node);
    }

    None
}

// Helper function to create a linked list from a vector
pub fn create_linked_list(values: Vec<i32>) -> Option<Rc<RefCell<ListNode>>> {
    if values.is_empty() {
        return None;
    }

    let head = Rc::new(RefCell::new(ListNode::new(values[0])));
    let mut current = head.clone();

    for i in 1..values.len() {
        let new_node = Rc::new(RefCell::new(ListNode::new(values[i])));
        current.borrow_mut().next = Some(new_node.clone());
        current = new_node;
    }

    Some(head)
}

// Helper function to convert linked list to vector for easy printing
pub fn linked_list_to_vec(head: Option<Rc<RefCell<ListNode>>>) -> Vec<i32> {
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
    fn test_remove_nth_node() {
        // Test 1: [1, 2, 3, 4, 5], n=2
        let head1 = create_linked_list(vec![1, 2, 3, 4, 5]);
        let result1 = remove_nth_node_two_pass(head1, 2);
        assert_eq!(linked_list_to_vec(result1), vec![1, 2, 3, 5]);

        // Test 2: [1], n=1
        let head2 = create_linked_list(vec![1]);
        let result2 = remove_nth_node_two_pass(head2, 1);
        assert_eq!(linked_list_to_vec(result2), vec![]);

        // Test 3: [1, 2], n=2
        let head3 = create_linked_list(vec![1, 2]);
        let result3 = remove_nth_node_two_pass(head3, 2);
        assert_eq!(linked_list_to_vec(result3), vec![2]);

        // Test 4: [1, 2], n=1
        let head4 = create_linked_list(vec![1, 2]);
        let result4 = remove_nth_node_two_pass(head4, 1);
        assert_eq!(linked_list_to_vec(result4), vec![1]);
    }
}
