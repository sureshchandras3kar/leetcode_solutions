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
 * Find maximum path sum in binary tree using post-order DFS.
 * A path can pass through any node (not just root to leaf).
 * Time: O(n), Space: O(h) for recursion stack
 * @param {TreeNode} root
 * @return {number}
 */
function maxPathSum(root) {
    let maxSum = Number.MIN_SAFE_INTEGER;

    function dfs(node) {
        if (!node) return 0;

        // Max gain from left and right subtrees (at least 0 if negative)
        const leftGain = Math.max(0, dfs(node.left));
        const rightGain = Math.max(0, dfs(node.right));

        // Max path through this node (may bend at this node)
        const maxPathThroughNode = node.val + leftGain + rightGain;
        maxSum = Math.max(maxSum, maxPathThroughNode);

        // Return max path extending downward from this node
        return node.val + Math.max(leftGain, rightGain);
    }

    dfs(root);
    return maxSum;
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
