class ListNode {
    constructor(val = 0, next = null) {
        this.val = val;
        this.next = next;
    }
}

/**
 * Merge two sorted linked lists iteratively.
 *
 * Time Complexity: O(m + n) where m and n are the lengths of list1 and list2
 * Space Complexity: O(1) excluding the output list
 *
 * @param {ListNode} list1
 * @param {ListNode} list2
 * @return {ListNode}
 */
function mergeTwoSortedListsIterative(list1, list2) {
    // Create a dummy node to simplify the code
    const dummy = new ListNode(0);
    let current = dummy;

    // Traverse both lists and append the smaller node
    while (list1 && list2) {
        if (list1.val <= list2.val) {
            current.next = list1;
            list1 = list1.next;
        } else {
            current.next = list2;
            list2 = list2.next;
        }
        current = current.next;
    }

    // Append the remaining nodes
    current.next = list1 || list2;

    return dummy.next;
}

// Helper function to create a linked list from an array
function createLinkedList(values) {
    if (values.length === 0) {
        return null;
    }
    const head = new ListNode(values[0]);
    let current = head;
    for (let i = 1; i < values.length; i++) {
        current.next = new ListNode(values[i]);
        current = current.next;
    }
    return head;
}

// Helper function to convert linked list to array for testing
function linkedListToArray(node) {
    const result = [];
    while (node) {
        result.push(node.val);
        node = node.next;
    }
    return result;
}

// Test cases
let list1 = createLinkedList([1, 2, 4]);
let list2 = createLinkedList([1, 3, 4]);
let result = mergeTwoSortedListsIterative(list1, list2);
console.log(linkedListToArray(result));  // [1, 1, 2, 3, 4, 4]

list1 = createLinkedList([]);
list2 = createLinkedList([]);
result = mergeTwoSortedListsIterative(list1, list2);
console.log(linkedListToArray(result));  // []

list1 = createLinkedList([]);
list2 = createLinkedList([0]);
result = mergeTwoSortedListsIterative(list1, list2);
console.log(linkedListToArray(result));  // [0]
