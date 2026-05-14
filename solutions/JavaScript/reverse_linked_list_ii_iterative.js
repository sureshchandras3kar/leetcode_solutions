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
 * Reverse a portion of a linked list from position left to right (1-indexed).
 *
 * Time Complexity: O(n) - single pass through the list
 * Space Complexity: O(1) - only pointer variables
 *
 * Strategy:
 * 1. Create a dummy node to handle edge case of reversing from head
 * 2. Advance prev pointer to position left-1
 * 3. Perform (right - left) reversals by moving nodes to the front of the sublist
 * 4. Return dummy.next
 *
 * @param {ListNode} head - The head of the linked list
 * @param {number} left - Starting position (1-indexed)
 * @param {number} right - Ending position (1-indexed)
 * @return {ListNode} - The head of the modified list
 */
function reverseBetween(head, left, right) {
    if (left === right) {
        return head;
    }

    // Create dummy node pointing to head
    const dummy = new ListNode(0, head);
    let prev = dummy;

    // Advance prev to position left-1
    for (let i = 0; i < left - 1; i++) {
        prev = prev.next;
    }

    // curr is the first node to reverse (at position left)
    let curr = prev.next;

    // Perform (right - left) reversals
    for (let i = 0; i < right - left; i++) {
        // nextTemp is the node we want to move to the front
        const nextTemp = curr.next;
        // Bypass nextTemp by connecting curr to nextTemp.next
        curr.next = nextTemp.next;
        // Move nextTemp to the front of the sublist
        nextTemp.next = prev.next;
        prev.next = nextTemp;
    }

    return dummy.next;
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
const head1 = createList([1, 2, 3, 4, 5]);
const result1 = reverseBetween(head1, 2, 4);
console.log('Test 1:', listToArray(result1));  // [1, 4, 3, 2, 5]

// Test 2: Single node (no reversal)
const head2 = createList([5]);
const result2 = reverseBetween(head2, 1, 1);
console.log('Test 2:', listToArray(result2));  // [5]

// Test 3: Reverse entire list
const head3 = createList([1, 2]);
const result3 = reverseBetween(head3, 1, 2);
console.log('Test 3:', listToArray(result3));  // [2, 1]

// Test 4: Reverse from start
const head4 = createList([1, 2, 3, 4, 5]);
const result4 = reverseBetween(head4, 1, 3);
console.log('Test 4:', listToArray(result4));  // [3, 2, 1, 4, 5]

// Test 5: Reverse at end
const head5 = createList([1, 2, 3, 4, 5]);
const result5 = reverseBetween(head5, 3, 5);
console.log('Test 5:', listToArray(result5));  // [1, 2, 5, 4, 3]

module.exports = reverseBetween;
