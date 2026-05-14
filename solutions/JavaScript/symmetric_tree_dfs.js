class TreeNode {
    constructor(val = 0, left = null, right = null) {
        this.val = val;
        this.left = left;
        this.right = right;
    }
}

function isSymmetricDFS(root) {
    /**
     * Check if a binary tree is symmetric using DFS recursion.
     *
     * Time Complexity: O(n) - visit each node once
     * Space Complexity: O(h) - recursion stack, where h is height
     */
    function isMirror(left, right) {
        // Both nodes are null - symmetric
        if (!left && !right) {
            return true;
        }

        // One node is null or values differ - not symmetric
        if (!left || !right) {
            return false;
        }
        if (left.val !== right.val) {
            return false;
        }

        // Recursively check: left's left with right's right
        // and left's right with right's left (mirror pattern)
        return isMirror(left.left, right.right) && isMirror(left.right, right.left);
    }

    return isMirror(root, root);
}

// Test cases
if (require.main === module) {
    // Example 1: Symmetric tree
    //       1
    //      / \
    //     2   2
    //    / \ / \
    //   3  4 4  3
    const root1 = new TreeNode(1);
    root1.left = new TreeNode(2);
    root1.right = new TreeNode(2);
    root1.left.left = new TreeNode(3);
    root1.left.right = new TreeNode(4);
    root1.right.left = new TreeNode(4);
    root1.right.right = new TreeNode(3);
    console.log(isSymmetricDFS(root1)); // true

    // Example 2: Not symmetric
    //       1
    //      / \
    //     2   2
    //      \   \
    //       3   3
    const root2 = new TreeNode(1);
    root2.left = new TreeNode(2);
    root2.right = new TreeNode(2);
    root2.left.right = new TreeNode(3);
    root2.right.right = new TreeNode(3);
    console.log(isSymmetricDFS(root2)); // false
}

module.exports = isSymmetricDFS;
