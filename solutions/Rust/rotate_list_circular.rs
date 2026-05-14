use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

/// Rotate a linked list to the right by k positions using the circular approach.
///
/// Approach:
/// 1. Calculate the effective rotation: k = k % length
/// 2. Create a circular list by connecting the tail to the head
/// 3. Find the new head position: walk (length - k) steps from the original head
/// 4. Break the circle at the appropriate point
///
/// Time: O(n) - single pass to find length and establish circle, walk (length - k) steps
/// Space: O(1) - only using pointers
pub fn rotate_list_circular(head: Option<Rc<RefCell<ListNode>>>, k: i32) -> Option<Rc<RefCell<ListNode>>> {
    let head = match head {
        Some(h) => h,
        None => return None,
    };

    // Check if next is None (single node or empty)
    {
        let head_ref = head.borrow();
        if head_ref.next.is_none() || k == 0 {
            return Some(head.clone());
        }
    }

    // Step 1: Find the length of the list and the tail
    let mut length = 1;
    let mut current = Some(head.clone());
    let mut tail_node = None;

    while let Some(node) = current {
        let next = node.borrow_mut().next.take();
        if next.is_none() {
            tail_node = Some(node.clone());
        } else {
            length += 1;
        }
        current = next;
    }

    // Step 2: Normalize k
    let k = k % length;
    if k == 0 {
        return Some(head);
    }

    // Step 3: Create a circular list
    if let Some(tail) = tail_node {
        tail.borrow_mut().next = Some(head.clone());

        // Step 4: Find the new head position
        let steps_to_walk = length - k;
        let mut current = Some(head.clone());
        for _ in 0..steps_to_walk - 1 {
            let node = current.unwrap();
            let next = node.borrow_mut().next.take();
            current = next;
        }

        // Step 5: Break the circle
        let break_node = current.unwrap();
        let new_head = break_node.borrow_mut().next.take();

        return new_head;
    }

    Some(head)
}

// Helper function to create a linked list from a vector
pub fn create_linked_list(values: Vec<i32>) -> Option<Rc<RefCell<ListNode>>> {
    if values.is_empty() {
        return None;
    }

    let head = Rc::new(RefCell::new(ListNode::new(values[0])));
    let mut current = head.clone();

    for val in values.into_iter().skip(1) {
        let new_node = Rc::new(RefCell::new(ListNode::new(val)));
        current.borrow_mut().next = Some(new_node.clone());
        current = new_node;
    }

    Some(head)
}

// Helper function to convert linked list to vector for easy printing
pub fn linked_list_to_vec(head: Option<Rc<RefCell<ListNode>>>) -> Vec<i32> {
    let mut result = vec![];
    let mut current = head;

    while let Some(node) = current {
        result.push(node.borrow().val);
        let next = node.borrow_mut().next.take();
        current = next;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_list_circular() {
        // Test 1: [1, 2, 3, 4, 5], k=2
        let head1 = create_linked_list(vec![1, 2, 3, 4, 5]);
        let result1 = rotate_list_circular(head1, 2);
        assert_eq!(linked_list_to_vec(result1), vec![4, 5, 1, 2, 3]);

        // Test 2: [0, 1, 2], k=4
        let head2 = create_linked_list(vec![0, 1, 2]);
        let result2 = rotate_list_circular(head2, 4);
        assert_eq!(linked_list_to_vec(result2), vec![2, 0, 1]);

        // Test 3: [1], k=1
        let head3 = create_linked_list(vec![1]);
        let result3 = rotate_list_circular(head3, 1);
        assert_eq!(linked_list_to_vec(result3), vec![1]);

        // Test 4: [1, 2], k=2
        let head4 = create_linked_list(vec![1, 2]);
        let result4 = rotate_list_circular(head4, 2);
        assert_eq!(linked_list_to_vec(result4), vec![1, 2]);

        // Test 5: [1, 2, 3, 4, 5], k=0
        let head5 = create_linked_list(vec![1, 2, 3, 4, 5]);
        let result5 = rotate_list_circular(head5, 0);
        assert_eq!(linked_list_to_vec(result5), vec![1, 2, 3, 4, 5]);
    }
}
