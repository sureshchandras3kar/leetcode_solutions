import java.util.ArrayList;
import java.util.List;

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
    private List<Integer> arr;
    private int index = 0;

    private void inorder(TreeNode node) {
        /**Pre-compute in-order traversal into list.*/
        if (node == null) return;

        inorder(node.left);
        arr.add(node.val);
        inorder(node.right);
    }

    /**
     * Binary Search Tree Iterator using pre-computed ArrayList.
     * Stores all in-order elements upfront.
     * Space: O(n), next() O(1), hasNext() O(1)
     */
    public BSTIterator(TreeNode root) {
        arr = new ArrayList<>();
        inorder(root);
    }

    public int next() {
        /**
         * Return next smallest element.
         * Time: O(1)
         */
        return arr.get(index++);
    }

    public boolean hasNext() {
        /**
         * Check if there are more elements.
         * Time: O(1)
         */
        return index < arr.size();
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
