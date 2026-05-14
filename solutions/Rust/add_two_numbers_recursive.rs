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
        ListNode { next: None, val }
    }
}

/// Add two numbers represented by linked lists in reverse order using recursion.
///
/// Time Complexity: O(max(m, n)) where m and n are the lengths of the lists
/// Space Complexity: O(max(m, n)) for the recursion call stack and output list
pub fn add_two_numbers(
    l1: Option<Rc<RefCell<ListNode>>>,
    l2: Option<Rc<RefCell<ListNode>>>,
) -> Option<Rc<RefCell<ListNode>>> {
    fn helper(
        l1: Option<Rc<RefCell<ListNode>>>,
        l2: Option<Rc<RefCell<ListNode>>>,
        carry: i32,
    ) -> Option<Rc<RefCell<ListNode>>> {
        // Base case: both lists are empty and no carry
        if l1.is_none() && l2.is_none() && carry == 0 {
            return None;
        }

        let val1 = l1.as_ref().map(|node| node.borrow().val).unwrap_or(0);
        let val2 = l2.as_ref().map(|node| node.borrow().val).unwrap_or(0);

        let total = val1 + val2 + carry;
        let digit = total % 10;
        let new_carry = total / 10;

        // Move to next nodes
        let next_l1 = l1.and_then(|node| node.borrow_mut().next.take());
        let next_l2 = l2.and_then(|node| node.borrow_mut().next.take());

        // Recursively build the rest of the list
        let next_node = helper(next_l1, next_l2, new_carry);

        // Create node with current digit
        Some(Rc::new(RefCell::new(ListNode {
            val: digit,
            next: next_node,
        })))
    }

    helper(l1, l2, 0)
}

// Helper function to create linked list from vector
fn create_linked_list(arr: Vec<i32>) -> Option<Rc<RefCell<ListNode>>> {
    if arr.is_empty() {
        return None;
    }

    let mut head = Rc::new(RefCell::new(ListNode::new(arr[0])));
    let mut current = head.clone();

    for &val in &arr[1..] {
        let new_node = Rc::new(RefCell::new(ListNode::new(val)));
        current.borrow_mut().next = Some(new_node.clone());
        current = new_node;
    }

    Some(head)
}

// Helper function to convert linked list to vector for printing
fn linked_list_to_vec(head: Option<Rc<RefCell<ListNode>>>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut current = head;

    while let Some(node) = current {
        let borrowed = node.borrow();
        result.push(borrowed.val);
        current = borrowed.next.clone();
    }

    result
}

fn main() {
    // Test case 1: [2,4,3] + [5,6,4] = [7,0,8] (342 + 465 = 807)
    let l1 = create_linked_list(vec![2, 4, 3]);
    let l2 = create_linked_list(vec![5, 6, 4]);
    let result = add_two_numbers(l1, l2);
    println!("Test 1: {:?}", linked_list_to_vec(result));  // [7, 0, 8]

    // Test case 2: [0] + [0] = [0]
    let l1 = create_linked_list(vec![0]);
    let l2 = create_linked_list(vec![0]);
    let result = add_two_numbers(l1, l2);
    println!("Test 2: {:?}", linked_list_to_vec(result));  // [0]

    // Test case 3: [9,9,9,9,9,9,9] + [9,9,9,9] = [8,9,9,9,0,0,0,1]
    let l1 = create_linked_list(vec![9, 9, 9, 9, 9, 9, 9]);
    let l2 = create_linked_list(vec![9, 9, 9, 9]);
    let result = add_two_numbers(l1, l2);
    println!("Test 3: {:?}", linked_list_to_vec(result));  // [8, 9, 9, 9, 0, 0, 0, 1]
}
