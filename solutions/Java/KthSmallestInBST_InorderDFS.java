public class KthSmallestInBST_InorderDFS {
    static class TreeNode {
        int val;
        TreeNode left;
        TreeNode right;

        TreeNode(int val) {
            this.val = val;
        }
    }

    private int count = 0;
    private int result = -1;

    private void inorder(TreeNode node, int k) {
        if (node == null || result != -1) return;

        // Traverse left subtree
        inorder(node.left, k);

        // Process current node
        count++;
        if (count == k) {
            result = node.val;
            return;
        }

        // Traverse right subtree
        inorder(node.right, k);
    }

    public int kthSmallest(TreeNode root, int k) {
        inorder(root, k);
        return result;
    }

    public static void main(String[] args) {
        TreeNode root = new TreeNode(3);
        root.left = new TreeNode(1);
        root.right = new TreeNode(4);
        root.left.right = new TreeNode(2);

        KthSmallestInBST_InorderDFS sol = new KthSmallestInBST_InorderDFS();
        System.out.println(sol.kthSmallest(root, 1));  // 1
        System.out.println(sol.kthSmallest(root, 3));  // 2
    }
}
