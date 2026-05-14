import java.util.HashMap;
import java.util.Map;

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
    private Map<Integer, Integer> inorderMap;

    private TreeNode build(int[] preorder, int preorderStart, int preorderEnd,
                          int inorderStart, int inorderEnd) {
        if (preorderStart > preorderEnd || inorderStart > inorderEnd) {
            return null;
        }

        // Root is the first element in preorder
        int rootVal = preorder[preorderStart];
        TreeNode root = new TreeNode(rootVal);

        // Find root position in inorder
        int rootInorderIdx = inorderMap.get(rootVal);

        // Number of elements in left subtree
        int leftSize = rootInorderIdx - inorderStart;

        // Build left subtree
        root.left = build(preorder, preorderStart + 1, preorderStart + leftSize,
                         inorderStart, rootInorderIdx - 1);

        // Build right subtree
        root.right = build(preorder, preorderStart + leftSize + 1, preorderEnd,
                          rootInorderIdx + 1, inorderEnd);

        return root;
    }

    public TreeNode buildTree(int[] preorder, int[] inorder) {
        if (preorder.length == 0 || inorder.length == 0) {
            return null;
        }

        // Create a map for quick lookup of indices in inorder
        inorderMap = new HashMap<>();
        for (int i = 0; i < inorder.length; i++) {
            inorderMap.put(inorder[i], i);
        }

        return build(preorder, 0, preorder.length - 1, 0, inorder.length - 1);
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
