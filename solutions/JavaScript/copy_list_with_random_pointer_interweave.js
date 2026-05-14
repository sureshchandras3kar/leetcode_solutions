/**
 * // Definition for a Node.
 * function Node(val, next, random) {
 *    this.val = val;
 *    this.next = next;
 *    this.random = random;
 * };
 */

function copyRandomList(head) {
    if (!head) return null;

    // First pass: create copies and interleave them
    // original -> copy -> original -> copy -> ...
    let current = head;
    while (current) {
        const copy = new Node(current.val);
        copy.next = current.next;
        current.next = copy;
        current = copy.next;
    }

    // Second pass: set random pointers for copy nodes
    current = head;
    while (current) {
        if (current.random) {
            current.next.random = current.random.next;
        }
        current = current.next.next;
    }

    // Third pass: restore original list and extract copy list
    current = head;
    const copyHead = head.next;
    while (current) {
        const copy = current.next;
        current.next = copy.next;
        copy.next = copy.next ? copy.next.next : null;
        current = current.next;
    }

    return copyHead;
}
