/**
 * Definition for singly-linked list node
 */
class ListNode {
    constructor(val = 0, next = null) {
        this.val = val;
        this.next = next;
    }
}

/**
 * Reverse a portion of a linked list from position left to right using recursion.
 *
 * Time Complexity: O(n) - traverse to left, then reverse to right
 * Space Complexity: O(n) - recursion call stack
 *
 * Strategy:
 * 1. If left == 1, call reverseN() to reverse the first 'right' nodes
 * 2. Otherwise, recursively process the next node with adjusted positions
 */

// Global variable to track the successor node
let successor = null;

/**
 * Reverse the first n nodes of a linked list.
 *
 * The global 'successor' variable tracks the node after the nth node,
 * which becomes the tail of the reversed sublist.
 *
 * @param {ListNode} head - Head of the list
 * @param {number} n - Number of nodes to reverse
 * @return {ListNode} - New head after reversal
 */
function reverseN(head, n) {
    if (n === 1) {
        // Base case: mark successor as the node after the first node
        successor = head.next;
        return head;
    }

    // Recursively reverse the rest
    const newHead = reverseN(head.next, n - 1);

    // At this point, head.next points to the (n-1)th node
    // We want to reverse: head.next.next = head
    head.next.next = head;

    // Connect head to successor (the node after position n)
    head.next = successor;

    return newHead;
}

/**
 * Reverse a portion of a linked list from position left to right.
 *
 * @param {ListNode} head - The head of the linked list
 * @param {number} left - Starting position (1-indexed)
 * @param {number} right - Ending position (1-indexed)
 * @return {ListNode} - The head of the modified list
 */
function reverseBetween(head, left, right) {
    if (left === 1) {
        return reverseN(head, right);
    } else {
        // Recursively process the rest of the list
        head.next = reverseBetween(head.next, left - 1, right - 1);
        return head;
    }
}

// Helper function to create a linked list from an array
function createList(values) {
    if (values.length === 0) return null;
    const head = new ListNode(values[0]);
    let current = head;
    for (let i = 1; i < values.length; i++) {
        current.next = new ListNode(values[i]);
        current = current.next;
    }
    return head;
}

// Helper function to convert linked list to array for comparison
function listToArray(head) {
    const result = [];
    while (head !== null) {
        result.push(head.val);
        head = head.next;
    }
    return result;
}

// Test cases
// Test 1: Reverse middle portion
let head1 = createList([1, 2, 3, 4, 5]);
successor = null;
let result1 = reverseBetween(head1, 2, 4);
console.log('Test 1:', listToArray(result1));  // [1, 4, 3, 2, 5]

// Test 2: Single node (no reversal)
let head2 = createList([5]);
successor = null;
let result2 = reverseBetween(head2, 1, 1);
console.log('Test 2:', listToArray(result2));  // [5]

// Test 3: Reverse entire list
let head3 = createList([1, 2]);
successor = null;
let result3 = reverseBetween(head3, 1, 2);
console.log('Test 3:', listToArray(result3));  // [2, 1]

// Test 4: Reverse from start
let head4 = createList([1, 2, 3, 4, 5]);
successor = null;
let result4 = reverseBetween(head4, 1, 3);
console.log('Test 4:', listToArray(result4));  // [3, 2, 1, 4, 5]

// Test 5: Reverse at end
let head5 = createList([1, 2, 3, 4, 5]);
successor = null;
let result5 = reverseBetween(head5, 3, 5);
console.log('Test 5:', listToArray(result5));  // [1, 2, 5, 4, 3]

module.exports = { reverseBetween, reverseN };
