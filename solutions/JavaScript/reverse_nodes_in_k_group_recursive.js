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
 * Reverse nodes in k-group recursively.
 *
 * Approach: Use recursion to handle each group independently.
 * - Check if there are k nodes remaining
 * - Reverse the first k nodes
 * - Recursively process the rest of the list
 * - Connect the reversed group to the result of the recursive call
 *
 * Time: O(n) - visit each node once
 * Space: O(n/k) - recursion depth is proportional to number of groups
 *
 * @param {ListNode} head
 * @param {number} k
 * @return {ListNode}
 */
var reverseKGroup = function (head, k) {
    // Find the k-th node
    let kth = head;
    for (let i = 0; i < k - 1 && kth; i++) {
        kth = kth.next;
    }

    // If no k-th node exists, cannot reverse, return original
    if (!kth) {
        return head;
    }

    // Save the next group's head (after k-th node)
    const nextGroupHead = kth.next;

    // Reverse from head to kth
    let prev = nextGroupHead;
    let curr = head;

    while (curr !== nextGroupHead) {
        const nextTemp = curr.next;
        curr.next = prev;
        prev = curr;
        curr = nextTemp;
    }

    // head is now the tail of the reversed group
    // prev is the new head (which was kth)
    // Recursively process the remaining list and connect
    head.next = reverseKGroup(nextGroupHead, k);

    return prev;  // Return the new head (which was kth)
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
