class ListNode {
    constructor(val = 0, next = null) {
        this.val = val;
        this.next = next;
    }
}

/**
 * Remove the nth node from the end of a linked list using two pointers in one
 * pass.
 *
 * Approach:
 * 1. Create a dummy node pointing to head (handles edge case of removing head)
 * 2. Use fast and slow pointers with a gap of n nodes between them
 * 3. Move both pointers until fast reaches the end
 * 4. Skip the target node by adjusting the slow pointer
 *
 * Time: O(L) single pass
 * Space: O(1)
 *
 * @param {ListNode} head
 * @param {number} n
 * @return {ListNode}
 */
function removeNthNodeTwoPointers(head, n) {
    // Create a dummy node to handle edge case of removing the head
    const dummy = new ListNode(0);
    dummy.next = head;
    let slow = dummy;
    let fast = dummy;

    // Move fast pointer n+1 steps ahead
    for (let i = 0; i <= n; i++) {
        if (!fast) return head;
        fast = fast.next;
    }

    // Move both pointers until fast reaches the end
    while (fast) {
        slow = slow.next;
        fast = fast.next;
    }

    // Remove the nth node
    slow.next = slow.next.next;
    return dummy.next;
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
    const result1 = removeNthNodeTwoPointers(head1, 2);
    console.log(linkedListToArray(result1));  // [1, 2, 3, 5]

    // Test 2: [1], n=1
    const head2 = createLinkedList([1]);
    const result2 = removeNthNodeTwoPointers(head2, 1);
    console.log(linkedListToArray(result2));  // []

    // Test 3: [1, 2], n=2
    const head3 = createLinkedList([1, 2]);
    const result3 = removeNthNodeTwoPointers(head3, 2);
    console.log(linkedListToArray(result3));  // [2]

    // Test 4: [1, 2], n=1
    const head4 = createLinkedList([1, 2]);
    const result4 = removeNthNodeTwoPointers(head4, 1);
    console.log(linkedListToArray(result4));  // [1]
}

module.exports = removeNthNodeTwoPointers;
