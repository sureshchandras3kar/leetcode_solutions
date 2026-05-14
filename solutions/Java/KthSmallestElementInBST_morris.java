class TreeNode {
    int val;
    TreeNode left;
    TreeNode right;

    TreeNode(int x) {
        val = x;
    }
}

class Solution {
    public int kthSmallest(TreeNode root, int k) {
        int count = 0;
        TreeNode current = root;

        while (current != null) {
            if (current.left == null) {
                // No left child: visit current
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
                    // First visit: create thread
                    predecessor.right = current;
                    current = current.left;
                } else {
                    // Second visit: remove thread, process current
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
