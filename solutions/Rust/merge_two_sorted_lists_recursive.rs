use std::rc::Rc;
use std::cell::RefCell;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

/**
 * Merge two sorted linked lists recursively.
 *
 * Time Complexity: O(m + n) where m and n are the lengths of list1 and list2
 * Space Complexity: O(m + n) due to the recursion call stack
 */
pub fn merge_two_sorted_lists_recursive(
    mut list1: Option<Box<ListNode>>,
    mut list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (list1, list2) {
        (None, _) => list2,
        (_, None) => list1,
        (Some(mut node1), Some(mut node2)) => {
            if node1.val <= node2.val {
                node1.next = merge_two_sorted_lists_recursive(node1.next, Some(node2));
                Some(node1)
            } else {
                node2.next = merge_two_sorted_lists_recursive(Some(node1), node2.next);
                Some(node2)
            }
        }
    }
}

// Helper function to create a linked list from a vector
pub fn create_linked_list(values: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for &val in values.iter().rev() {
        let mut node = Box::new(ListNode::new(val));
        node.next = head;
        head = Some(node);
    }
    head
}

// Helper function to convert linked list to vector for testing
pub fn linked_list_to_vec(mut node: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = Vec::new();
    while let Some(n) = node {
        result.push(n.val);
        node = n.next;
    }
    result
}

fn main() {
    let list1 = create_linked_list(vec![1, 2, 4]);
    let list2 = create_linked_list(vec![1, 3, 4]);
    let result = merge_two_sorted_lists_recursive(list1, list2);
    println!("{:?}", linked_list_to_vec(result));  // [1, 1, 2, 3, 4, 4]

    let list1 = create_linked_list(vec![]);
    let list2 = create_linked_list(vec![]);
    let result = merge_two_sorted_lists_recursive(list1, list2);
    println!("{:?}", linked_list_to_vec(result));  // []

    let list1 = create_linked_list(vec![]);
    let list2 = create_linked_list(vec![0]);
    let result = merge_two_sorted_lists_recursive(list1, list2);
    println!("{:?}", linked_list_to_vec(result));  // [0]
}
