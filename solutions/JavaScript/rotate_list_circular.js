class ListNode {
    constructor(val = 0, next = null) {
        this.val = val;
        this.next = next;
    }
}

/**
 * Rotate a linked list to the right by k positions using the circular approach.
 *
 * Approach:
 * 1. Calculate the effective rotation: k = k % length
 * 2. Create a circular list by connecting the tail to the head
 * 3. Find the new head position: walk (length - k) steps from the original head
 * 4. Break the circle at the appropriate point
 *
 * Time: O(n) - single pass to find length and establish circle, walk (length - k) steps
 * Space: O(1) - only using pointers
 */
function rotateListCircular(head, k) {
    if (!head || !head.next || k === 0) {
        return head;
    }

    // Step 1: Find the length of the list and the tail
    let length = 0;
    let tail = head;
    while (tail.next) {
        length++;
        tail = tail.next;
    }
    length++;  // Add 1 for the tail node itself

    // Step 2: Normalize k
    k = k % length;
    if (k === 0) {
        return head;
    }

    // Step 3: Create a circular list
    tail.next = head;

    // Step 4: Find the new head position
    // After rotation by k, the new head is at position (length - k)
    // We need to walk (length - k) steps from the original head
    const stepsToWalk = length - k;
    let current = head;
    for (let i = 0; i < stepsToWalk - 1; i++) {
        current = current.next;
    }

    // Step 5: Break the circle
    const newHead = current.next;
    current.next = null;

    return newHead;
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
    // Test 1: [1, 2, 3, 4, 5], k=2
    let head1 = createLinkedList([1, 2, 3, 4, 5]);
    let result1 = rotateListCircular(head1, 2);
    console.log(linkedListToArray(result1));  // [4, 5, 1, 2, 3]

    // Test 2: [0, 1, 2], k=4
    let head2 = createLinkedList([0, 1, 2]);
    let result2 = rotateListCircular(head2, 4);
    console.log(linkedListToArray(result2));  // [2, 0, 1]

    // Test 3: [1], k=1
    let head3 = createLinkedList([1]);
    let result3 = rotateListCircular(head3, 1);
    console.log(linkedListToArray(result3));  // [1]

    // Test 4: [1, 2], k=2
    let head4 = createLinkedList([1, 2]);
    let result4 = rotateListCircular(head4, 2);
    console.log(linkedListToArray(result4));  // [1, 2]

    // Test 5: [1, 2, 3, 4, 5], k=0
    let head5 = createLinkedList([1, 2, 3, 4, 5]);
    let result5 = rotateListCircular(head5, 0);
    console.log(linkedListToArray(result5));  // [1, 2, 3, 4, 5]
}

module.exports = { rotateListCircular, ListNode, createLinkedList, linkedListToArray };
