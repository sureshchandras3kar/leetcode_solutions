use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
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
 * Reverse nodes in k-group iteratively.
 *
 * Approach: Reverse each group of k nodes in-place using iteration.
 * - Find the k-th node to determine the group boundary
 * - Reverse the group from current to k-th node
 * - Link the reversed group back to the previous group
 * - Move to the next group and repeat
 *
 * Time: O(n) - visit each node once
 * Space: O(1) - only pointers, no extra structures
 */
pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    fn get_kth_node(mut node: Option<&mut Box<ListNode>>, k: i32) -> Option<&mut Box<ListNode>> {
        let mut k = k;
        loop {
            match node {
                Some(n) if k > 1 => {
                    k -= 1;
                    node = n.next.as_mut().map(|next| next);
                }
                Some(n) => return Some(n),
                None => return None,
            }
        }
    }

    let mut dummy = Box::new(ListNode::new(0));
    dummy.next = head;

    let mut prev_group = &mut dummy;

    loop {
        // Check if there are at least k nodes remaining
        let kth_node = {
            let mut curr = &mut prev_group.next;
            let mut count = k;
            while count > 1 && curr.is_some() {
                curr = &mut curr.as_mut().unwrap().next;
                count -= 1;
            }
            if curr.is_some() && count == 1 {
                curr.is_some()
            } else {
                false
            }
        };

        if !kth_node {
            break;
        }

        // Manually reverse k nodes
        let mut group_start = prev_group.next.take();
        let mut kth = &mut group_start;
        for _ in 1..k {
            kth = &mut kth.as_mut().unwrap().next;
        }

        let next_group_head = kth.as_mut().unwrap().next.take();

        // Perform reversal
        let mut prev = next_group_head;
        let mut curr = group_start;

        for _ in 0..k {
            let next_tmp = curr.as_mut().unwrap().next.take();
            curr.as_mut().unwrap().next = prev;
            prev = curr;
            curr = next_tmp;
        }

        let group_end = prev;
        prev_group.next = group_end;

        // Move prev_group forward
        for _ in 0..k {
            prev_group = &mut prev_group.next.as_mut().unwrap();
        }
    }

    dummy.next
}

fn create_list(values: Vec<i32>) -> Option<Box<ListNode>> {
    if values.is_empty() {
        return None;
    }
    let mut head = Box::new(ListNode::new(values[0]));
    let mut current = &mut head;
    for i in 1..values.len() {
        current.next = Some(Box::new(ListNode::new(values[i])));
        current = current.next.as_mut().unwrap();
    }
    Some(head)
}

fn print_list(head: Option<Box<ListNode>>) {
    let mut result = vec![];
    let mut current = head;
    while let Some(node) = current {
        result.push(node.val.to_string());
        current = node.next;
    }
    println!("[{}]", result.join(", "));
}

fn main() {
    let head1 = create_list(vec![1, 2, 3, 4, 5]);
    print_list(reverse_k_group(head1, 2));  // [2, 1, 4, 3, 5]

    let head2 = create_list(vec![1, 2, 3, 4, 5]);
    print_list(reverse_k_group(head2, 3));  // [3, 2, 1, 4, 5]

    let head3 = create_list(vec![1]);
    print_list(reverse_k_group(head3, 1));  // [1]
}
