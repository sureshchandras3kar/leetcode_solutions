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
 * Check if tree has root-to-leaf path summing to targetSum using iterative BFS.
 * Queue stores [node, current_sum] pairs.
 * Time: O(n), Space: O(w) where w is max width
 * @param {TreeNode} root
 * @param {number} targetSum
 * @return {boolean}
 */
function hasPathSum(root, targetSum) {
    if (!root) return false;

    const queue = [[root, root.val]];

    while (queue.length > 0) {
        const [node, currentSum] = queue.shift();

        // Check leaf node
        if (!node.left && !node.right && currentSum === targetSum) {
            return true;
        }

        if (node.left) {
            queue.push([node.left, currentSum + node.left.val]);
        }
        if (node.right) {
            queue.push([node.right, currentSum + node.right.val]);
        }
    }

    return false;
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
