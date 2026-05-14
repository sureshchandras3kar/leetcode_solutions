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
 * Add two numbers represented by linked lists in reverse order using recursion.
 *
 * Time Complexity: O(max(m, n)) where m and n are the lengths of the lists
 * Space Complexity: O(max(m, n)) for the recursion call stack and output list
 *
 * @param {ListNode} l1
 * @param {ListNode} l2
 * @return {ListNode}
 */
const addTwoNumbers = (l1, l2) => {
    const helper = (l1, l2, carry) => {
        // Base case: both lists are empty and no carry
        if (!l1 && !l2 && carry === 0) {
            return null;
        }

        const val1 = l1 ? l1.val : 0;
        const val2 = l2 ? l2.val : 0;

        const total = val1 + val2 + carry;
        const digit = total % 10;
        const newCarry = Math.floor(total / 10);

        // Move to next nodes
        const nextL1 = l1 ? l1.next : null;
        const nextL2 = l2 ? l2.next : null;

        // Recursively build the rest of the list
        const nextNode = helper(nextL1, nextL2, newCarry);

        // Create node with current digit
        return new ListNode(digit, nextNode);
    };

    return helper(l1, l2, 0);
};

// Helper function to create linked list from array
const createLinkedList = (arr) => {
    if (arr.length === 0) return null;
    const head = new ListNode(arr[0]);
    let current = head;
    for (let i = 1; i < arr.length; i++) {
        current.next = new ListNode(arr[i]);
        current = current.next;
    }
    return head;
};

// Helper function to convert linked list to array for printing
const linkedListToArray = (head) => {
    const result = [];
    let current = head;
    while (current) {
        result.push(current.val);
        current = current.next;
    }
    return result;
};

// Test cases
if (require.main === module) {
    // Test case 1: [2,4,3] + [5,6,4] = [7,0,8] (342 + 465 = 807)
    let l1 = createLinkedList([2, 4, 3]);
    let l2 = createLinkedList([5, 6, 4]);
    let result = addTwoNumbers(l1, l2);
    console.log("Test 1:", linkedListToArray(result));  // [7, 0, 8]

    // Test case 2: [0] + [0] = [0]
    l1 = createLinkedList([0]);
    l2 = createLinkedList([0]);
    result = addTwoNumbers(l1, l2);
    console.log("Test 2:", linkedListToArray(result));  // [0]

    // Test case 3: [9,9,9,9,9,9,9] + [9,9,9,9] = [8,9,9,9,0,0,0,1]
    l1 = createLinkedList([9, 9, 9, 9, 9, 9, 9]);
    l2 = createLinkedList([9, 9, 9, 9]);
    result = addTwoNumbers(l1, l2);
    console.log("Test 3:", linkedListToArray(result));  // [8, 9, 9, 9, 0, 0, 0, 1]
}

module.exports = addTwoNumbers;
