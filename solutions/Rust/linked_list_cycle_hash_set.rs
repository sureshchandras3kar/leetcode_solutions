use std::collections::HashSet;

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

struct Solution;

impl Solution {
    pub fn has_cycle(head: Option<Box<ListNode>>) -> bool {
        let mut seen = HashSet::new();
        let mut current = head;

        while let Some(node) = current {
            let node_ptr = &node as *const _;
            if !seen.insert(node_ptr as usize) {
                return true;
            }
            current = node.next;
        }

        false
    }
}

fn main() {
    // Test case: Basic structure without cycle
    let node1 = Box::new(ListNode::new(1));
    let mut node2 = Box::new(ListNode::new(2));
    node2.next = None;

    let mut node3 = Box::new(ListNode::new(3));
    node3.next = Some(node2);

    println!("Linked List Cycle - Hash Set Approach");
}
