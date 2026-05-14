use std::collections::HashMap;

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
 * Remove all nodes with duplicate values from sorted linked list using hash set.
 *
 * Approach: Count frequency of each value, then rebuild list with unique values only.
 * Time: O(n) — two passes through the list
 * Space: O(n) — hash map stores frequencies
 */
pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return None;
    }

    // Count frequencies
    let mut freq = HashMap::new();
    let mut current = head.as_ref();
    while let Some(node) = current {
        *freq.entry(node.val).or_insert(0) += 1;
        current = node.next.as_ref();
    }

    // Build result list with unique values only
    let mut head = head;
    let mut dummy = Box::new(ListNode::new(0));
    let mut prev = &mut dummy;

    while let Some(mut node) = head {
        head = node.next.take();
        if freq[&node.val] == 1 {
            // Keep unique node
            prev.next = Some(node);
            prev = prev.next.as_mut().unwrap();
        }
        // Skip duplicate node (don't add to result)
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
