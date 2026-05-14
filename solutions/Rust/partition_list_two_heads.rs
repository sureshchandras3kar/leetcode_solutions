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

/// Partition linked list into two groups: values < x and values >= x.
/// Uses two separate list heads and merges them at the end.
///
/// Time Complexity: O(n) where n is the number of nodes
/// Space Complexity: O(1) excluding the output list
pub fn partition(
    head: Option<Rc<RefCell<ListNode>>>,
    x: i32,
) -> Option<Rc<RefCell<ListNode>>> {
    // Create dummy nodes for two lists
    let mut smaller_dummy = Box::new(ListNode::new(0));
    let mut larger_dummy = Box::new(ListNode::new(0));

    // Pointers to build the two lists
    let mut smaller_ptr = &mut smaller_dummy.next;
    let mut larger_ptr = &mut larger_dummy.next;

    // Partition nodes into two lists
    let mut current = head;
    while let Some(node) = current {
        let next = node.borrow_mut().next.take();
        if node.borrow().val < x {
            *smaller_ptr = Some(node);
            smaller_ptr = &mut smaller_ptr.as_mut().unwrap().borrow_mut().next;
        } else {
            *larger_ptr = Some(node);
            larger_ptr = &mut larger_ptr.as_mut().unwrap().borrow_mut().next;
        }
        current = next;
    }

    // Connect the two lists
    *smaller_ptr = larger_dummy.next;

    smaller_dummy.next
}

// Helper function to create a linked list from a vector
fn create_linked_list(values: Vec<i32>) -> Option<Rc<RefCell<ListNode>>> {
    if values.is_empty() {
        return None;
    }
    let mut head = ListNode::new(values[0]);
    let mut current = &mut head;
    for val in &values[1..] {
        let new_node = ListNode::new(*val);
        current.next = Some(Rc::new(RefCell::new(new_node)));
        current = &mut current.next.as_mut().unwrap().borrow_mut();
    }
    Some(Rc::new(RefCell::new(head)))
}

// Helper function to convert linked list to vector for easy testing
fn linked_list_to_vec(node: Option<Rc<RefCell<ListNode>>>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut current = node;
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
    fn test_partition_list() {
        let head = create_linked_list(vec![1, 4, 3, 2, 5, 2]);
        let result = partition(head, 3);
        assert_eq!(linked_list_to_vec(result), vec![1, 2, 2, 4, 3, 5]);

        let head = create_linked_list(vec![2, 1]);
        let result = partition(head, 2);
        assert_eq!(linked_list_to_vec(result), vec![1, 2]);

        let head = create_linked_list(vec![5, 3, 1, 4, 2]);
        let result = partition(head, 3);
        assert_eq!(linked_list_to_vec(result), vec![1, 2, 5, 3, 4]);
    }
}

fn main() {
    let head = create_linked_list(vec![1, 4, 3, 2, 5, 2]);
    let result = partition(head, 3);
    println!("{:?}", linked_list_to_vec(result));  // [1, 2, 2, 4, 3, 5]

    let head = create_linked_list(vec![2, 1]);
    let result = partition(head, 2);
    println!("{:?}", linked_list_to_vec(result));  // [1, 2]

    let head = create_linked_list(vec![5, 3, 1, 4, 2]);
    let result = partition(head, 3);
    println!("{:?}", linked_list_to_vec(result));  // [1, 2, 5, 3, 4]
}
