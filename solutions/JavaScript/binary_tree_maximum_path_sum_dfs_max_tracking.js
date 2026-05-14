/**
 * Definition for a binary tree node.
 */
class TreeNode {
    constructor(val = 0, left = null, right = null) {
        this.val = val;
        this.left = left;
        this.right = right;
    }
}

/**
 * Find maximum path sum using DFS with mutable max tracking.
 * Maintains maxSum as variable updated during traversal.
 * Time: O(n), Space: O(h) for recursion stack
 * @param {TreeNode} root
 * @return {number}
 */
function maxPathSum(root) {
    const maxSum = [Number.MIN_SAFE_INTEGER];

    function dfs(node) {
        if (!node) return 0;

        // Max single-path sum extending from this node
        const leftSum = Math.max(0, dfs(node.left));
        const rightSum = Math.max(0, dfs(node.right));

        // Path bending at this node
        const pathSum = node.val + leftSum + rightSum;
        maxSum[0] = Math.max(maxSum[0], pathSum);

        // Return best single path extending downward
        return node.val + Math.max(leftSum, rightSum);
    }

    dfs(root);
    return maxSum[0];
}

// Test cases
const root1 = new TreeNode(1);
root1.left = new TreeNode(2);
root1.right = new TreeNode(3);

console.log("Example 1 max path:", maxPathSum(root1));  // 6 (2 -> 1 -> 3)

const root2 = new TreeNode(-10);
root2.left = new TreeNode(9);
root2.right = new TreeNode(20);
root2.right.left = new TreeNode(15);
root2.right.right = new TreeNode(7);

console.log("Example 2 max path:", maxPathSum(root2));  // 42 (15 -> 20 -> 7)
