/**
 * Definition for singly-linked list node.
 */
class ListNode {
    constructor(val = 0, next = null) {
        this.val = val;
        this.next = next;
    }
}

/**
 * Remove all nodes with duplicate values from sorted linked list in single pass.
 *
 * Approach: Use a dummy node and prev pointer. When we find duplicates, skip all
 * nodes with that value by advancing current and updating prev.next.
 * Time: O(n) — single pass through the list
 * Space: O(1) — only pointer variables
 *
 * @param {ListNode} head
 * @return {ListNode}
 */
function deleteDuplicates(head) {
    if (!head) return null;

    // Create dummy node to handle edge case where head is duplicate
    const dummy = new ListNode(0);
    dummy.next = head;
    let prev = dummy;
    let current = head;

    while (current) {
        // Check if current node is the start of a duplicate group
        if (current.next && current.val === current.next.val) {
            // Skip all nodes with the same value
            const value = current.val;
            while (current && current.val === value) {
                current = current.next;
            }
            // Link prev to the first non-duplicate node
            prev.next = current;
        } else {
            // Current node is unique, keep it
            prev = current;
            current = current.next;
        }
    }

    return dummy.next;
}

/**
 * Helper function to create linked list from array
 */
function createList(arr) {
    if (arr.length === 0) return null;
    const head = new ListNode(arr[0]);
    let current = head;
    for (let i = 1; i < arr.length; i++) {
        current.next = new ListNode(arr[i]);
        current = current.next;
    }
    return head;
}

/**
 * Helper function to print linked list
 */
function printList(head) {
    let result = '';
    while (head) {
        result += head.val + ' -> ';
        head = head.next;
    }
    result += 'null';
    console.log(result);
}

// Test cases
console.log('Test 1:');
const head1 = createList([1, 2, 3, 3, 4, 4, 5]);
printList(deleteDuplicates(head1));  // 1 -> 2 -> 5 -> null

console.log('Test 2:');
const head2 = createList([1, 1, 1, 2, 3]);
printList(deleteDuplicates(head2));  // 2 -> 3 -> null

console.log('Test 3:');
const head3 = createList([1, 1]);
printList(deleteDuplicates(head3));  // null

module.exports = { deleteDuplicates, ListNode };
