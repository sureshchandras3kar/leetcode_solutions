import java.util.HashMap;
import java.util.Map;

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

        // Map original nodes to their copies
        Map<Node, Node> nodeMap = new HashMap<>();

        // First pass: create all copy nodes
        Node current = head;
        while (current != null) {
            nodeMap.put(current, new Node(current.val));
            current = current.next;
        }

        // Second pass: set next and random pointers
        current = head;
        while (current != null) {
            Node copyNode = nodeMap.get(current);
            copyNode.next = current.next != null ? nodeMap.get(current.next) : null;
            copyNode.random = current.random != null ? nodeMap.get(current.random) : null;
            current = current.next;
        }

        return nodeMap.get(head);
    }
}
