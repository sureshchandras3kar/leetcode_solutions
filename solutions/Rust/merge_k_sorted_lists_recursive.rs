use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::Ordering;

#[derive(Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    if lists.is_empty() {
        return None;
    }
    partition(&lists, 0, lists.len() - 1)
}

fn partition(lists: &Vec<Option<Box<ListNode>>>, left: usize, right: i32) -> Option<Box<ListNode>> {
    if left as i32 == right {
        lists[left].clone()
    } else if left as i32 > right {
        None
    } else {
        let mid = (left as i32 + right) / 2;
        let l1 = partition(lists, left, mid);
        let l2 = partition(lists, mid as usize + 1, right);
        merge(l1, l2)
    }
}

fn merge(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (Some(mut node1), Some(mut node2)) => {
            if node1.val <= node2.val {
                node1.next = merge(node1.next.take(), Some(node2));
                Some(node1)
            } else {
                node2.next = merge(Some(node1), node2.next.take());
                Some(node2)
            }
        }
        (Some(node), None) => Some(node),
        (None, Some(node)) => Some(node),
        (None, None) => None,
    }
}

fn main() {}
