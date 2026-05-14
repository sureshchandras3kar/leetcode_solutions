public class KthSmallestInBST_MorrisInorder {
    static class TreeNode {
        int val;
        TreeNode left;
        TreeNode right;

        TreeNode(int val) {
            this.val = val;
        }
    }

    public int kthSmallest(TreeNode root, int k) {
        int count = 0;
        TreeNode current = root;

        while (current != null) {
            if (current.left == null) {
                // Left is null, process current node
                count++;
                if (count == k) {
                    return current.val;
                }
                current = current.right;
            } else {
                // Find in-order predecessor
                TreeNode predecessor = current.left;
                while (predecessor.right != null && predecessor.right != current) {
                    predecessor = predecessor.right;
                }

                if (predecessor.right == null) {
                    // Create link to current node
                    predecessor.right = current;
                    current = current.left;
                } else {
                    // Link exists, remove it and process current
                    predecessor.right = null;
                    count++;
                    if (count == k) {
                        return current.val;
                    }
                    current = current.right;
                }
            }
        }

        return -1;
    }

    public static void main(String[] args) {
        TreeNode root = new TreeNode(3);
        root.left = new TreeNode(1);
        root.right = new TreeNode(4);
        root.left.right = new TreeNode(2);

        KthSmallestInBST_MorrisInorder sol = new KthSmallestInBST_MorrisInorder();
        System.out.println(sol.kthSmallest(root, 1));  // 1
        System.out.println(sol.kthSmallest(root, 3));  // 2
    }
}
