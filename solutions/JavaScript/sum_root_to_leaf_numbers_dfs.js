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
 * Sum all root-to-leaf numbers using DFS.
 * Build number by appending digits as we traverse down.
 * Time: O(n), Space: O(h) for recursion stack
 * @param {TreeNode} root
 * @return {number}
 */
function sumNumbers(root) {
    function dfs(node, currentSum) {
        if (!node) return 0;

        // Build number: multiply by 10 and add current digit
        currentSum = currentSum * 10 + node.val;

        // Leaf node: return the complete number
        if (!node.left && !node.right) {
            return currentSum;
        }

        // Recursively process children and sum
        return dfs(node.left, currentSum) + dfs(node.right, currentSum);
    }

    return dfs(root, 0);
}

// Test cases
const root1 = new TreeNode(1);
root1.left = new TreeNode(2);
root1.right = new TreeNode(3);

console.log("Example 1 sum:", sumNumbers(root1));  // 25 (12 + 13)

const root2 = new TreeNode(4);
root2.left = new TreeNode(9);
root2.right = new TreeNode(0);
root2.left.left = new TreeNode(5);
root2.left.right = new TreeNode(1);

console.log("Example 2 sum:", sumNumbers(root2));  // 1026 (495 + 491 + 40)
