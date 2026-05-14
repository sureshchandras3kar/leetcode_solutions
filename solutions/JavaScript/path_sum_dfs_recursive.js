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
 * Check if tree has root-to-leaf path summing to targetSum using recursive DFS.
 * Time: O(n), Space: O(h) for recursion stack
 * @param {TreeNode} root
 * @param {number} targetSum
 * @return {boolean}
 */
function hasPathSum(root, targetSum) {
    if (!root) return false;

    // Leaf node check
    if (!root.left && !root.right) {
        return root.val === targetSum;
    }

    // Subtract current value and check left and right subtrees
    const remaining = targetSum - root.val;
    return hasPathSum(root.left, remaining) || hasPathSum(root.right, remaining);
}

// Test cases
const root = new TreeNode(5);
root.left = new TreeNode(4);
root.left.left = new TreeNode(11);
root.left.left.left = new TreeNode(7);
root.left.left.right = new TreeNode(2);
root.right = new TreeNode(8);
root.right.left = new TreeNode(13);
root.right.right = new TreeNode(4);
root.right.right.right = new TreeNode(1);

console.log("Example 1 (target 22):", hasPathSum(root, 22));  // true
console.log("Example 1 (target 20):", hasPathSum(root, 20));  // false
