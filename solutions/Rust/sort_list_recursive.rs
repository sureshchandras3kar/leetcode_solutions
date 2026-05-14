use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match head {
        None => None,
        Some(mut node) if node.next.is_none() => Some(node),
        Some(mut head) => {
            let mut slow = &mut head;
            let mut fast = &head;
            let mut prev = None;

            while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
                prev = Some(slow);
                if let Some(s) = slow {
                    slow = &mut s.next;
                }
                if let Some(f) = fast {
                    fast = &f.next.as_ref().map(|n| &n.next);
                }
            }

            // Split the list (simplified version)
            let right = if let Some(mut p) = prev {
                p.take()
            } else {
                Some(Box::new(ListNode { val: 0, next: None }))
            };

            let left = Some(head);
            merge(sort_list(left), sort_list(right))
        }
    }
}

fn merge(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (Some(mut l1), Some(mut l2)) => {
            if l1.val <= l2.val {
                l1.next = merge(l1.next.take(), Some(l2));
                Some(l1)
            } else {
                l2.next = merge(Some(l1), l2.next.take());
                Some(l2)
            }
        }
        (Some(l1), None) => Some(l1),
        (None, Some(l2)) => Some(l2),
        (None, None) => None,
    }
}

fn main() {}
