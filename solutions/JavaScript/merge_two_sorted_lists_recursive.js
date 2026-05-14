class ListNode {
    constructor(val = 0, next = null) {
        this.val = val;
        this.next = next;
    }
}

/**
 * Merge two sorted linked lists recursively.
 *
 * Time Complexity: O(m + n) where m and n are the lengths of list1 and list2
 * Space Complexity: O(m + n) due to the recursion call stack
 *
 * @param {ListNode} list1
 * @param {ListNode} list2
 * @return {ListNode}
 */
function mergeTwoSortedListsRecursive(list1, list2) {
    // Base cases: if one list is empty, return the other
    if (!list1) {
        return list2;
    }
    if (!list2) {
        return list1;
    }

    // Compare the values and recursively merge
    if (list1.val <= list2.val) {
        list1.next = mergeTwoSortedListsRecursive(list1.next, list2);
        return list1;
    } else {
        list2.next = mergeTwoSortedListsRecursive(list1, list2.next);
        return list2;
    }
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
let result = mergeTwoSortedListsRecursive(list1, list2);
console.log(linkedListToArray(result));  // [1, 1, 2, 3, 4, 4]

list1 = createLinkedList([]);
list2 = createLinkedList([]);
result = mergeTwoSortedListsRecursive(list1, list2);
console.log(linkedListToArray(result));  // []

list1 = createLinkedList([]);
list2 = createLinkedList([0]);
result = mergeTwoSortedListsRecursive(list1, list2);
console.log(linkedListToArray(result));  // [0]
