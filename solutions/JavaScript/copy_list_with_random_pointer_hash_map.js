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

    // Map original nodes to their copies
    const nodeMap = new Map();

    // First pass: create all copy nodes
    let current = head;
    while (current) {
        nodeMap.set(current, new Node(current.val));
        current = current.next;
    }

    // Second pass: set next and random pointers
    current = head;
    while (current) {
        const copyNode = nodeMap.get(current);
        copyNode.next = current.next ? nodeMap.get(current.next) : null;
        copyNode.random = current.random ? nodeMap.get(current.random) : null;
        current = current.next;
    }

    return nodeMap.get(head);
}
