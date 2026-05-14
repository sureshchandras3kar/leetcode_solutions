use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    head.and_then(|h| {
        let mut length = 0;
        let mut curr = Some(&h);
        while let Some(n) = curr {
            length += 1;
            curr = n.next.as_ref();
        }

        let mut dummy = ListNode { val: 0, next: Some(h) };
        let mut size = 1;

        while size < length {
            let mut prev = &mut dummy;
            let mut curr = prev.next.take();

            while curr.is_some() {
                let l1 = curr.take();
                let (l1, l1_len) = split(l1, size);
                let (l2, _) = split(l2_after(l1.as_ref(), l1_len), size);

                prev.next = merge(l1, l2);
                while prev.next.is_some() {
                    prev = &mut prev.next.as_mut().unwrap();
                }
                curr = l2_after(prev, 1);
            }
            size *= 2;
        }

        dummy.next
    })
}

fn split(mut list: Option<Box<ListNode>>, size: usize) -> (Option<Box<ListNode>>, usize) {
    let mut len = 0;
    let mut curr = &mut list;

    while len < size && curr.is_some() {
        if let Some(n) = curr {
            curr = &mut n.next;
            len += 1;
        }
    }

    let right = curr.take();
    (list, len)
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

fn l2_after(node: &Option<Box<ListNode>>, steps: usize) -> Option<Box<ListNode>> {
    None // Simplified
}

fn main() {}
