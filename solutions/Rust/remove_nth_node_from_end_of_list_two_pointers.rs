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
Remove the nth node from the end of a linked list using two pointers in one pass.

Approach:
1. Create a dummy node pointing to head (handles edge case of removing head)
2. Use fast and slow pointers with a gap of n nodes between them
3. Move both pointers until fast reaches the end
4. Skip the target node by adjusting the slow pointer

Time: O(L) single pass
Space: O(1)
*/
pub fn remove_nth_node_two_pointers(
    head: Option<Rc<RefCell<ListNode>>>,
    n: i32,
) -> Option<Rc<RefCell<ListNode>>> {
    let dummy = Rc::new(RefCell::new(ListNode::new(0)));
    dummy.borrow_mut().next = head;

    let mut slow = dummy.clone();
    let mut fast = dummy.clone();

    // Move fast pointer n+1 steps ahead
    for _ in 0..=n {
        if fast.borrow().next.is_none() {
            return dummy.borrow_mut().next.take();
        }
        let next = fast.borrow_mut().next.take();
        if let Some(node) = next {
            fast = node;
        }
    }

    // Move both pointers until fast reaches the end
    loop {
        if fast.borrow().next.is_none() {
            break;
        }
        let slow_next = slow.borrow_mut().next.take();
        if let Some(node) = slow_next {
            slow = node;
        }
        let fast_next = fast.borrow_mut().next.take();
        if let Some(node) = fast_next {
            fast = node;
        }
    }

    // Remove the nth node
    if let Some(next_node) = slow.borrow_mut().next.take() {
        slow.borrow_mut().next = next_node.borrow_mut().next.take();
    }

    dummy.borrow_mut().next.take()
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
        let result1 = remove_nth_node_two_pointers(head1, 2);
        assert_eq!(linked_list_to_vec(result1), vec![1, 2, 3, 5]);

        // Test 2: [1], n=1
        let head2 = create_linked_list(vec![1]);
        let result2 = remove_nth_node_two_pointers(head2, 1);
        assert_eq!(linked_list_to_vec(result2), vec![]);

        // Test 3: [1, 2], n=2
        let head3 = create_linked_list(vec![1, 2]);
        let result3 = remove_nth_node_two_pointers(head3, 2);
        assert_eq!(linked_list_to_vec(result3), vec![2]);

        // Test 4: [1, 2], n=1
        let head4 = create_linked_list(vec![1, 2]);
        let result4 = remove_nth_node_two_pointers(head4, 1);
        assert_eq!(linked_list_to_vec(result4), vec![1]);
    }
}
