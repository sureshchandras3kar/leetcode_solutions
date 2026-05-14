function TreeNode(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
}

/**
 * DFS Recursive approach to invert a binary tree.
 * Recursively swap left and right children for each node.
 *
 * Time Complexity: O(n) - visit each node once
 * Space Complexity: O(h) - recursion stack depth (h = height)
 *
 * @param {TreeNode} root
 * @return {TreeNode}
 */
var invertTree = function(root) {
    if (root === null) {
        return null;
    }

    // Swap left and right children
    [root.left, root.right] = [root.right, root.left];

    // Recursively invert left and right subtrees
    invertTree(root.left);
    invertTree(root.right);

    return root;
};

// Test case
if (require.main === module) {
    // Create tree:     1
    //                /   \
    //               2     3
    const root = new TreeNode(1);
    root.left = new TreeNode(2);
    root.right = new TreeNode(3);

    invertTree(root);

    // Expected:        1
    //                /   \
    //               3     2
    console.log(`Root: ${root.val}`);  // 1
    console.log(`Left: ${root.left.val}, Right: ${root.right.val}`);  // 3, 2
}

module.exports = invertTree;
