import java.util.Queue;
import java.util.LinkedList;

class TreeNode {
    int val;
    TreeNode left;
    TreeNode right;

    TreeNode() {}
    TreeNode(int val) {
        this.val = val;
    }
}

class Pair {
    TreeNode node;
    int sum;

    Pair(TreeNode node, int sum) {
        this.node = node;
        this.sum = sum;
    }
}

class Solution {
    /**
     * Sum all root-to-leaf numbers using iterative BFS.
     * Queue stores (node, current_number) pairs.
     * Time: O(n), Space: O(w) where w is max width
     */
    public int sumNumbers(TreeNode root) {
        if (root == null) return 0;

        Queue<Pair> queue = new LinkedList<>();
        queue.add(new Pair(root, root.val));
        int total = 0;

        while (!queue.isEmpty()) {
            Pair pair = queue.poll();
            TreeNode node = pair.node;
            int currentSum = pair.sum;

            // Leaf node: add to total
            if (node.left == null && node.right == null) {
                total += currentSum;
                continue;
            }

            if (node.left != null) {
                queue.add(new Pair(node.left, currentSum * 10 + node.left.val));
            }
            if (node.right != null) {
                queue.add(new Pair(node.right, currentSum * 10 + node.right.val));
            }
        }

        return total;
    }
}

class Main {
    public static void main(String[] args) {
        // Example 1: [1,2,3]
        TreeNode root = new TreeNode(1);
        root.left = new TreeNode(2);
        root.right = new TreeNode(3);

        Solution sol = new Solution();
        System.out.println("Example 1 sum: " + sol.sumNumbers(root));  // 25 (12 + 13)

        // Example 2: [4,9,0,5,1]
        TreeNode root2 = new TreeNode(4);
        root2.left = new TreeNode(9);
        root2.right = new TreeNode(0);
        root2.left.left = new TreeNode(5);
        root2.left.right = new TreeNode(1);

        System.out.println("Example 2 sum: " + sol.sumNumbers(root2));  // 1026 (495 + 491 + 40)
    }
}
