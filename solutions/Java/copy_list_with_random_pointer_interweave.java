class Node {
    int val;
    Node next;
    Node random;

    public Node(int val) {
        this.val = val;
        this.next = null;
        this.random = null;
    }
}

class Solution {
    public Node copyRandomList(Node head) {
        if (head == null) return null;

        // First pass: create copies and interleave them
        // original -> copy -> original -> copy -> ...
        Node current = head;
        while (current != null) {
            Node copy = new Node(current.val);
            copy.next = current.next;
            current.next = copy;
            current = copy.next;
        }

        // Second pass: set random pointers for copy nodes
        current = head;
        while (current != null) {
            if (current.random != null) {
                current.next.random = current.random.next;
            }
            current = current.next.next;
        }

        // Third pass: restore original list and extract copy list
        current = head;
        Node copyHead = head.next;
        while (current != null) {
            Node copy = current.next;
            current.next = copy.next;
            copy.next = copy.next != null ? copy.next.next : null;
            current = current.next;
        }

        return copyHead;
    }
}
