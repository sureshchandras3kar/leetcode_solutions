import java.util.Arrays;

class TreeNode {
    int val;
    TreeNode left;
    TreeNode right;
    TreeNode() {}
    TreeNode(int val) { this.val = val; }
    TreeNode(int val, TreeNode left, TreeNode right) {
        this.val = val;
        this.left = left;
        this.right = right;
    }
}

class Solution {
    private int preorderIdx;

    private TreeNode build(int[] preorder, int[] inorder,
                          int inorderStart, int inorderEnd) {
        if (inorderStart > inorderEnd || preorderIdx >= preorder.length) {
            return null;
        }

        // Root is the current element in preorder
        int rootVal = preorder[preorderIdx];
        preorderIdx++;
        TreeNode root = new TreeNode(rootVal);

        // Find root position in inorder
        int rootInorderIdx = -1;
        for (int i = inorderStart; i <= inorderEnd; i++) {
            if (inorder[i] == rootVal) {
                rootInorderIdx = i;
                break;
            }
        }

        // Build left subtree
        root.left = build(preorder, inorder, inorderStart, rootInorderIdx - 1);

        // Build right subtree
        root.right = build(preorder, inorder, rootInorderIdx + 1, inorderEnd);

        return root;
    }

    public TreeNode buildTree(int[] preorder, int[] inorder) {
        if (preorder.length == 0 || inorder.length == 0) {
            return null;
        }

        preorderIdx = 0;
        return build(preorder, inorder, 0, inorder.length - 1);
    }
}

// Test cases
class Main {
    public static void main(String[] args) {
        Solution sol = new Solution();

        // Test 1: [3,9,20,15,7], [9,3,15,20,7]
        int[] preorder1 = {3, 9, 20, 15, 7};
        int[] inorder1 = {9, 3, 15, 20, 7};
        TreeNode root1 = sol.buildTree(preorder1, inorder1);
        System.out.println("Test 1 - Root: " + root1.val);  // 3

        // Test 2: [1], [1]
        int[] preorder2 = {1};
        int[] inorder2 = {1};
        TreeNode root2 = sol.buildTree(preorder2, inorder2);
        System.out.println("Test 2 - Root: " + root2.val);  // 1
    }
}
