class ListNode {
    constructor(val = 0, next = null) {
        this.val = val;
        this.next = next;
    }
}

/**
 * Partition linked list into two groups: values < x and values >= x.
 * Uses two separate list heads and merges them at the end.
 *
 * Time Complexity: O(n) where n is the number of nodes
 * Space Complexity: O(1) excluding the output list
 */
function partitionListTwoHeads(head, x) {
    // Create dummy nodes for two lists
    const smaller_dummy = new ListNode(0);
    const larger_dummy = new ListNode(0);

    // Pointers to build the two lists
    let smaller_ptr = smaller_dummy;
    let larger_ptr = larger_dummy;

    // Partition nodes into two lists
    let current = head;
    while (current) {
        if (current.val < x) {
            smaller_ptr.next = current;
            smaller_ptr = smaller_ptr.next;
        } else {
            larger_ptr.next = current;
            larger_ptr = larger_ptr.next;
        }
        current = current.next;
    }

    // Close the larger list to avoid cycles
    larger_ptr.next = null;

    // Connect the two lists
    smaller_ptr.next = larger_dummy.next;

    return smaller_dummy.next;
}

// Helper function to create a linked list from an array
function createLinkedList(values) {
    if (values.length === 0) {
        return null;
    }
    let head = new ListNode(values[0]);
    let current = head;
    for (let i = 1; i < values.length; i++) {
        current.next = new ListNode(values[i]);
        current = current.next;
    }
    return head;
}

// Helper function to convert linked list to array for easy testing
function linkedListToArray(node) {
    const result = [];
    while (node) {
        result.push(node.val);
        node = node.next;
    }
    return result;
}

// Test cases
let head = createLinkedList([1, 4, 3, 2, 5, 2]);
let result = partitionListTwoHeads(head, 3);
console.log(linkedListToArray(result));  // [1, 2, 2, 4, 3, 5]

head = createLinkedList([2, 1]);
result = partitionListTwoHeads(head, 2);
console.log(linkedListToArray(result));  // [1, 2]

head = createLinkedList([5, 3, 1, 4, 2]);
result = partitionListTwoHeads(head, 3);
console.log(linkedListToArray(result));  // [1, 2, 5, 3, 4]
