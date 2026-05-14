use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl PartialEq for ListNode {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl Eq for ListNode {}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.val.cmp(&self.val)
    }
}

fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut heap = BinaryHeap::new();
    for lst in lists {
        if let Some(node) = lst {
            heap.push(node);
        }
    }

    let mut dummy = ListNode { val: 0, next: None };
    let mut curr = &mut dummy;

    while let Some(mut node) = heap.pop() {
        if let Some(next) = node.next.take() {
            heap.push(next);
        }
        curr.next = Some(node);
        curr = curr.next.as_mut().unwrap();
    }

    dummy.next
}

fn main() {}
