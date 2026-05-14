class ListNode {
    constructor(val = 0, next = null) {
        this.val = val;
        this.next = next;
    }
}

/**
 * Rotate a linked list to the right by k positions using the break point approach.
 *
 * Approach:
 * 1. Calculate the effective rotation: k = k % length
 * 2. Find the break point: the node at position (length - k - 1)
 * 3. Perform rotation by breaking the list at the break point
 *
 * Time: O(n) - single pass to find length, single pass to find break point
 * Space: O(1) - only using pointers
 */
function rotateListBreakPoint(head, k) {
    if (!head || !head.next || k === 0) {
        return head;
    }

    // Step 1: Find the length of the list
    let length = 0;
    let current = head;
    while (current) {
        length++;
        current = current.next;
    }

    // Step 2: Normalize k (handle cases where k > length)
    k = k % length;
    if (k === 0) {
        return head;
    }

    // Step 3: Find the break point (node before rotation point)
    // We need to find the (length - k - 1)th node
    current = head;
    for (let i = 0; i < length - k - 1; i++) {
        current = current.next;
    }

    // Step 4: Perform rotation
    // The new head is current.next
    const newHead = current.next;
    current.next = null;

    // Find the tail of the new list and connect it back to the old head
    let tail = newHead;
    while (tail.next) {
        tail = tail.next;
    }
    tail.next = head;

    return newHead;
}

// Helper function to create a linked list from an array
function createLinkedList(values) {
    if (values.length === 0) return null;
    const head = new ListNode(values[0]);
    let current = head;
    for (let i = 1; i < values.length; i++) {
        current.next = new ListNode(values[i]);
        current = current.next;
    }
    return head;
}

// Helper function to convert linked list to array for easy printing
function linkedListToArray(head) {
    const result = [];
    let current = head;
    while (current) {
        result.push(current.val);
        current = current.next;
    }
    return result;
}

// Test cases
if (require.main === module) {
    // Test 1: [1, 2, 3, 4, 5], k=2
    let head1 = createLinkedList([1, 2, 3, 4, 5]);
    let result1 = rotateListBreakPoint(head1, 2);
    console.log(linkedListToArray(result1));  // [4, 5, 1, 2, 3]

    // Test 2: [0, 1, 2], k=4
    let head2 = createLinkedList([0, 1, 2]);
    let result2 = rotateListBreakPoint(head2, 4);
    console.log(linkedListToArray(result2));  // [2, 0, 1]

    // Test 3: [1], k=1
    let head3 = createLinkedList([1]);
    let result3 = rotateListBreakPoint(head3, 1);
    console.log(linkedListToArray(result3));  // [1]

    // Test 4: [1, 2], k=2
    let head4 = createLinkedList([1, 2]);
    let result4 = rotateListBreakPoint(head4, 2);
    console.log(linkedListToArray(result4));  // [1, 2]

    // Test 5: [1, 2, 3, 4, 5], k=0
    let head5 = createLinkedList([1, 2, 3, 4, 5]);
    let result5 = rotateListBreakPoint(head5, 0);
    console.log(linkedListToArray(result5));  // [1, 2, 3, 4, 5]
}

module.exports = { rotateListBreakPoint, ListNode, createLinkedList, linkedListToArray };
