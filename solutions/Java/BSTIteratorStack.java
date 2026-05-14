import java.util.Stack;

class TreeNode {
    int val;
    TreeNode left;
    TreeNode right;

    TreeNode() {}
    TreeNode(int val) {
        this.val = val;
    }
}

class BSTIterator {
    private Stack<TreeNode> stack;

    private void pushLeft(TreeNode node) {
        /**Push all left nodes onto stack.*/
        while (node != null) {
            stack.push(node);
            node = node.left;
        }
    }

    /**
     * Binary Search Tree Iterator using stack for in-order traversal.
     * Implements lazy evaluation: next() O(1) amortized, hasNext() O(1).
     * Space: O(h) where h is height
     */
    public BSTIterator(TreeNode root) {
        stack = new Stack<>();
        pushLeft(root);
    }

    public int next() {
        /**
         * Return next smallest element.
         * Time: O(1) amortized
         */
        TreeNode node = stack.pop();

        if (node.right != null) {
            pushLeft(node.right);
        }

        return node.val;
    }

    public boolean hasNext() {
        /**
         * Check if there are more elements.
         * Time: O(1)
         */
        return !stack.isEmpty();
    }
}

class Main {
    public static void main(String[] args) {
        // Example: [7,3,15,null,null,9,20]
        TreeNode root = new TreeNode(7);
        root.left = new TreeNode(3);
        root.right = new TreeNode(15);
        root.right.left = new TreeNode(9);
        root.right.right = new TreeNode(20);

        BSTIterator iterator = new BSTIterator(root);
        while (iterator.hasNext()) {
            System.out.print(iterator.next() + " ");
        }
        System.out.println();  // 3 7 9 15 20
    }
}
