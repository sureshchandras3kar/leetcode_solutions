#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

/**
 * Remove all nodes with duplicate values from sorted linked list in single pass.
 *
 * Approach: Use a dummy node and prev pointer. When we find duplicates, skip all
 * nodes with that value by advancing current and updating prev.next.
 * Time: O(n) — single pass through the list
 * Space: O(1) — only pointer variables
 */
pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return None;
    }

    // Create dummy node to handle edge case where head is duplicate
    let mut dummy = Box::new(ListNode::new(0));
    dummy.next = head;
    let mut prev = &mut dummy;
    let mut current = prev.next.take();

    while let Some(mut node) = current {
        current = node.next.take();

        // Check if current node is the start of a duplicate group
        if let Some(next_node) = &current {
            if node.val == next_node.val {
                // Skip all nodes with the same value
                let value = node.val;
                while let Some(n) = current {
                    if n.val != value {
                        current = Some(n);
                        break;
                    }
                    current = n.next;
                }
                // Link prev to the first non-duplicate node
                prev.next = current;
                current = prev.next.take();
                continue;
            }
        }

        // Current node is unique, keep it
        prev.next = Some(node);
        prev = prev.next.as_mut().unwrap();
    }

    dummy.next.take()
}

fn main() {
    // Test case 1: [1,2,3,3,4,4,5]
    let head1 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode {
                                val: 5,
                                next: None,
                            })),
                        })),
                    })),
                })),
            })),
        })),
    }));
    println!("Test 1: {:?}", delete_duplicates(head1));  // 1 -> 2 -> 5

    // Test case 2: [1,1,1,2,3]
    let head2 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: None,
                    })),
                })),
            })),
        })),
    }));
    println!("Test 2: {:?}", delete_duplicates(head2));  // 2 -> 3

    // Test case 3: [1,1]
    let head3 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 1,
            next: None,
        })),
    }));
    println!("Test 3: {:?}", delete_duplicates(head3));  // null
}
