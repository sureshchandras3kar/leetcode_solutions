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
/// Rearranges nodes in a single pass without creating separate lists.
///
/// Time Complexity: O(n) where n is the number of nodes
/// Space Complexity: O(1) excluding the output list
pub fn partition(
    head: Option<Rc<RefCell<ListNode>>>,
    x: i32,
) -> Option<Rc<RefCell<ListNode>>> {
    // Create a dummy node to simplify edge cases
    let mut dummy = Box::new(ListNode::new(0));
    dummy.next = head;

    let mut dummy_ref = dummy;
    let mut prev = &mut dummy_ref as *mut Box<ListNode>;
    let mut current = unsafe { &mut (*prev).next };

    // Find the node just before the partition point
    while let Some(ref mut node) = *current {
        if node.borrow().val < x {
            prev = &mut **current as *mut _;
            current = &mut node.borrow_mut().next;
        } else {
            break;
        }
    }

    // If all nodes are less than x or list is empty, return as is
    if current.is_none() {
        return dummy_ref.next;
    }

    // Now we need to insert nodes with value < x before the partition point
    let mut partition_prev = prev;
    let mut run = current;

    while let Some(ref mut node) = *run {
        if node.borrow().val < x {
            // Remove run from its position
            let node_next = node.borrow_mut().next.take();
            *current = node_next;

            // Insert run before the partition point
            unsafe {
                let partition_prev_ref = &mut *partition_prev;
                let new_node = run.take().unwrap();
                let mut new_node_mut = new_node.borrow_mut();
                new_node_mut.next = partition_prev_ref.next.take();
                partition_prev_ref.next = Some(new_node.clone());
                drop(new_node_mut);

                partition_prev = &mut new_node.borrow_mut() as *mut _;
                run = &mut current;
            }
        } else {
            // Move forward
            prev = current as *mut Box<ListNode>;
            current = &mut node.borrow_mut().next;
            run = current;
        }
    }

    dummy_ref.next
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
