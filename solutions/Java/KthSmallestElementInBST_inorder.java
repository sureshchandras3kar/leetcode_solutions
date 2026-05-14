class TreeNode {
    int val;
    TreeNode left;
    TreeNode right;

    TreeNode(int x) {
        val = x;
    }
}

class Solution {
    private int count = 0;
    private int result = -1;

    private void inorder(TreeNode node, int k) {
        if (node == null || result != -1) return;

        inorder(node.left, k);

        count++;
        if (count == k) {
            result = node.val;
            return;
        }

        inorder(node.right, k);
    }

    public int kthSmallest(TreeNode root, int k) {
        inorder(root, k);
        return result;
    }
}

// Test
class Main {
    public static void main(String[] args) {
        TreeNode root = new TreeNode(3);
        root.left = new TreeNode(1);
        root.right = new TreeNode(4);
        root.left.right = new TreeNode(2);

        Solution sol = new Solution();
        System.out.println(sol.kthSmallest(root, 1));  // 1
        System.out.println(sol.kthSmallest(root, 3));  // 2
    }
}
