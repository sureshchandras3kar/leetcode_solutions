class ListNode {
    constructor(val = 0, next = null) {
        this.val = val;
        this.next = next;
    }
}

/**
 * Partition linked list into two groups: values < x and values >= x.
 * Rearranges nodes in a single pass without creating separate lists.
 *
 * Time Complexity: O(n) where n is the number of nodes
 * Space Complexity: O(1) excluding the output list
 */
function partitionListSingleRearrange(head, x) {
    // Create a dummy node to simplify edge cases
    const dummy = new ListNode(0);
    dummy.next = head;

    // Find the node just before the partition point
    let prev = dummy;
    let current = head;

    while (current && current.val < x) {
        prev = current;
        current = current.next;
    }

    // Now prev is the last node with value < x (or dummy if all nodes >= x)
    // current is the first node with value >= x (or null if all nodes < x)

    // If all nodes are less than x or list is empty, return as is
    if (!current) {
        return dummy.next;
    }

    // Now we need to insert nodes with value < x before the partition point
    let partition_prev = prev;  // Last node of < x group
    let run = current;  // Start from the partition point

    while (run) {
        if (run.val < x) {
            // Remove run from its position
            current.next = run.next;

            // Insert run before the partition point
            run.next = partition_prev.next;
            partition_prev.next = run;

            // Move partition_prev forward
            partition_prev = run;

            // Current stays the same to check the next node
            run = current.next;
        } else {
            // Move forward
            current = run;
            run = run.next;
        }
    }

    return dummy.next;
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
let result = partitionListSingleRearrange(head, 3);
console.log(linkedListToArray(result));  // [1, 2, 2, 4, 3, 5]

head = createLinkedList([2, 1]);
result = partitionListSingleRearrange(head, 2);
console.log(linkedListToArray(result));  // [1, 2]

head = createLinkedList([5, 3, 1, 4, 2]);
result = partitionListSingleRearrange(head, 3);
console.log(linkedListToArray(result));  // [1, 2, 5, 3, 4]
