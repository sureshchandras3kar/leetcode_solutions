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
     * Populates next pointers using pre-computed links without queue.
     * Uses next pointers of parent level to traverse current level.
     * Time: O(n), Space: O(1)
     */
    public Node connect(Node root) {
        if (root == null) return root;

        Node leftmost = root;

        while (leftmost != null) {
            Node prev = null;
            Node current = leftmost;

            while (current != null) {
                if (current.left != null) {
                    if (prev != null) {
                        prev.next = current.left;
                    }
                    prev = current.left;
                }

                if (current.right != null) {
                    if (prev != null) {
                        prev.next = current.right;
                    }
                    prev = current.right;
                }

                current = current.next;
            }

            leftmost = leftmost.left;
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
        System.out.println("Example 1: Tree with next pointers connected via pre-computed links");
    }
}
