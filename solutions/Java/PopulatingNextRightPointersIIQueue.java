import java.util.Queue;
import java.util.LinkedList;

class Node {
    public int val;
    public Node left;
    public Node right;
    public Node next;

    public Node() {}
    public Node(int _val) {
        val = _val;
    }
}

class Solution {
    /**
     * Populates next pointers using level-order BFS queue.
     * Time: O(n), Space: O(w) where w is max width
     */
    public Node connect(Node root) {
        if (root == null) return root;

        Queue<Node> queue = new LinkedList<>();
        queue.add(root);

        while (!queue.isEmpty()) {
            int levelSize = queue.size();
            Node prev = null;

            for (int i = 0; i < levelSize; i++) {
                Node node = queue.poll();

                if (prev != null) {
                    prev.next = node;
                }
                prev = node;

                if (node.left != null) queue.add(node.left);
                if (node.right != null) queue.add(node.right);
            }
        }

        return root;
    }
}

class Main {
    public static void main(String[] args) {
        // Example 1: Tree with next pointers connected
        Node root = new Node(1);
        root.left = new Node(2);
        root.right = new Node(3);
        root.left.left = new Node(4);
        root.left.right = new Node(5);
        root.right.right = new Node(7);

        Solution sol = new Solution();
        sol.connect(root);
        System.out.println("Example 1: Tree with next pointers connected via queue");
    }
}
