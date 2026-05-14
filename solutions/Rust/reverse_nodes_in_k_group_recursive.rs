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
 * Reverse nodes in k-group recursively.
 *
 * Approach: Use recursion to handle each group independently.
 * - Check if there are k nodes remaining
 * - Reverse the first k nodes
 * - Recursively process the rest of the list
 * - Connect the reversed group to the result of the recursive call
 *
 * Time: O(n) - visit each node once
 * Space: O(n/k) - recursion depth is proportional to number of groups
 */
pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    // Find the k-th node
    let mut kth = head.as_ref();
    for _ in 0..k - 1 {
        kth = match kth {
            Some(node) => node.next.as_ref(),
            None => return head,
        };
    }

    // If no k-th node exists, cannot reverse, return original
    if kth.is_none() {
        return head;
    }

    // Get the node after k-th (next group's head)
    let mut head = head;
    let next_group_head = {
        let mut curr = head.as_mut().unwrap();
        for _ in 0..k - 1 {
            curr = curr.next.as_mut().unwrap();
        }
        curr.next.take()
    };

    // Reverse the first k nodes
    let mut prev = next_group_head.clone();
    let mut curr = head;

    for _ in 0..k {
        let next_tmp = curr.as_mut().unwrap().next.take();
        curr.as_mut().unwrap().next = prev;
        prev = curr;
        curr = next_tmp;
    }

    // curr is now None (we've processed k nodes)
    // prev is the new head of the reversed group
    // Connect to recursive result
    let mut result = prev;
    let mut node = result.as_mut().unwrap();
    for _ in 0..k - 1 {
        node = node.next.as_mut().unwrap();
    }
    node.next = reverse_k_group(next_group_head, k);

    result
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
