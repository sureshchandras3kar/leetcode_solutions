use std::collections::VecDeque;

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
        let mut slow = head.clone();
        let mut fast = head;

        while let Some(fast_node) = fast {
            // Move fast pointer twice
            fast = fast_node.next.clone();
            if let Some(fast_node2) = fast {
                fast = fast_node2.next.clone();
            } else {
                return false;
            }

            // Move slow pointer once
            if let Some(slow_node) = slow {
                slow = slow_node.next.clone();
            } else {
                return false;
            }

            // Check if they point to the same node
            if slow == fast {
                return true;
            }
        }

        false
    }
}

fn main() {
    // Test case 1: Cycle exists
    let mut node1 = Box::new(ListNode::new(3));
    let mut node2 = Box::new(ListNode::new(2));
    let mut node3 = Box::new(ListNode::new(0));
    let node4 = Box::new(ListNode::new(-4));

    node3.next = Some(node4);
    node2.next = Some(node3);
    node1.next = Some(node2);
    // Note: Cannot create actual cycle with Rust's ownership model in this simple example

    println!("Linked List Cycle - Floyd's Algorithm");
}
