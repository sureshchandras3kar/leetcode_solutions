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
 * Reverse nodes in k-group iteratively.
 *
 * Approach: Reverse each group of k nodes in-place using iteration.
 * - Find the k-th node to determine the group boundary
 * - Reverse the group from current to k-th node
 * - Link the reversed group back to the previous group
 * - Move to the next group and repeat
 *
 * Time: O(n) - visit each node once
 * Space: O(1) - only pointers, no extra structures
 *
 * @param {ListNode} head
 * @param {number} k
 * @return {ListNode}
 */
var reverseKGroup = function (head, k) {
    const getKthNode = (node, k) => {
        while (node && k > 1) {
            node = node.next;
            k--;
        }
        return node;
    };

    // Dummy node to simplify logic
    const dummy = new ListNode(0);
    dummy.next = head;

    let prevGroup = dummy;

    while (true) {
        // Check if there are at least k nodes remaining
        const kthNode = getKthNode(prevGroup, k + 1);
        if (!kthNode) {
            break;
        }

        // Mark the start and end of the current group
        const groupStart = prevGroup.next;
        const groupEnd = kthNode;

        // Save the next group's start
        const nextGroupStart = groupEnd.next;

        // Reverse the current group
        let prev = nextGroupStart;
        let curr = groupStart;

        while (curr !== nextGroupStart) {
            const nextTemp = curr.next;
            curr.next = prev;
            prev = curr;
            curr = nextTemp;
        }

        // Link the previous group to the reversed current group
        prevGroup.next = groupEnd;
        prevGroup = groupStart;
    }

    return dummy.next;
};

// Helper functions for testing
const createList = (values) => {
    if (values.length === 0) return null;
    let head = new ListNode(values[0]);
    let current = head;
    for (let i = 1; i < values.length; i++) {
        current.next = new ListNode(values[i]);
        current = current.next;
    }
    return head;
};

const printList = (head) => {
    const result = [];
    while (head) {
        result.push(head.val);
        head = head.next;
    }
    console.log("[" + result.join(", ") + "]");
};

// Test
let head1 = createList([1, 2, 3, 4, 5]);
printList(reverseKGroup(head1, 2));  // [2, 1, 4, 3, 5]

let head2 = createList([1, 2, 3, 4, 5]);
printList(reverseKGroup(head2, 3));  // [3, 2, 1, 4, 5]

let head3 = createList([1]);
printList(reverseKGroup(head3, 1));  // [1]
