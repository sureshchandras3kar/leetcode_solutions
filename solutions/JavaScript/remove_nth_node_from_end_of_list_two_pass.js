class ListNode {
    constructor(val = 0, next = null) {
        this.val = val;
        this.next = next;
    }
}

/**
 * Remove the nth node from the end of a linked list using two passes.
 *
 * Approach:
 * 1. First pass: count the total number of nodes
 * 2. Second pass: find and skip the nth node from the end
 *
 * Time: O(L) + O(L-n) = O(L)
 * Space: O(1)
 *
 * @param {ListNode} head
 * @param {number} n
 * @return {ListNode}
 */
function removeNthNodeTwoPass(head, n) {
    // First pass: count the total nodes
    let length = 0;
    let current = head;
    while (current) {
        length++;
        current = current.next;
    }

    // Edge case: removing the head node
    if (length === n) {
        return head.next;
    }

    // Second pass: find the node before the one to remove
    current = head;
    for (let i = 0; i < length - n - 1; i++) {
        current = current.next;
    }

    // Remove the nth node
    current.next = current.next.next;
    return head;
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
    // Test 1: [1, 2, 3, 4, 5], n=2
    const head1 = createLinkedList([1, 2, 3, 4, 5]);
    const result1 = removeNthNodeTwoPass(head1, 2);
    console.log(linkedListToArray(result1));  // [1, 2, 3, 5]

    // Test 2: [1], n=1
    const head2 = createLinkedList([1]);
    const result2 = removeNthNodeTwoPass(head2, 1);
    console.log(linkedListToArray(result2));  // []

    // Test 3: [1, 2], n=2
    const head3 = createLinkedList([1, 2]);
    const result3 = removeNthNodeTwoPass(head3, 2);
    console.log(linkedListToArray(result3));  // [2]

    // Test 4: [1, 2], n=1
    const head4 = createLinkedList([1, 2]);
    const result4 = removeNthNodeTwoPass(head4, 1);
    console.log(linkedListToArray(result4));  // [1]
}

module.exports = removeNthNodeTwoPass;
