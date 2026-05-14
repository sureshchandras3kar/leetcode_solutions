function ListNode(val) {
    this.val = val;
    this.next = null;
}

/**
 * @param {ListNode} head
 * @return {boolean}
 */
var hasCycle = function(head) {
    const seen = new Set();
    let current = head;

    while (current) {
        if (seen.has(current)) {
            return true;
        }
        seen.add(current);
        current = current.next;
    }

    return false;
};

// Test cases
if (require.main === module) {
    // Test case 1: Cycle exists
    const node1 = new ListNode(3);
    const node2 = new ListNode(2);
    const node3 = new ListNode(0);
    const node4 = new ListNode(-4);
    node1.next = node2;
    node2.next = node3;
    node3.next = node4;
    node4.next = node2;  // Cycle
    console.log("Cycle exists:", hasCycle(node1));  // true

    // Test case 2: No cycle
    const node5 = new ListNode(1);
    const node6 = new ListNode(2);
    node5.next = node6;
    console.log("Cycle exists:", hasCycle(node5));  // false

    // Test case 3: Single node
    const node7 = new ListNode(1);
    console.log("Cycle exists:", hasCycle(node7));  // false
}

module.exports = hasCycle;
