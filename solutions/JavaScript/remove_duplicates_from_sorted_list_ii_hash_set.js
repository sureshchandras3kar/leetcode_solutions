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
 * Remove all nodes with duplicate values from sorted linked list using hash set.
 *
 * Approach: Count frequency of each value, then rebuild list with unique values only.
 * Time: O(n) — two passes through the list
 * Space: O(n) — hash map stores frequencies
 *
 * @param {ListNode} head
 * @return {ListNode}
 */
function deleteDuplicates(head) {
    if (!head) return null;

    // Count frequencies
    const freq = new Map();
    let current = head;
    while (current) {
        freq.set(current.val, (freq.get(current.val) || 0) + 1);
        current = current.next;
    }

    // Build result list with unique values only
    const dummy = new ListNode(0);
    dummy.next = head;
    let prev = dummy;
    current = head;

    while (current) {
        if (freq.get(current.val) === 1) {
            // Keep unique node
            prev = current;
            current = current.next;
        } else {
            // Skip duplicate node
            prev.next = current.next;
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
