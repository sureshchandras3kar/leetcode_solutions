function TreeNode(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
}

/**
 * BFS Iterative approach to invert a binary tree.
 * Uses a queue to visit nodes level by level, swapping children.
 *
 * Time Complexity: O(n) - visit each node once
 * Space Complexity: O(w) - w = max width of tree (nodes at widest level)
 *
 * @param {TreeNode} root
 * @return {TreeNode}
 */
var invertTree = function(root) {
    if (root === null) {
        return root;
    }

    const queue = [root];
    let front = 0;

    while (front < queue.length) {
        const node = queue[front++];

        // Swap left and right children
        [node.left, node.right] = [node.right, node.left];

        // Add children to queue for processing
        if (node.left !== null) {
            queue.push(node.left);
        }
        if (node.right !== null) {
            queue.push(node.right);
        }
    }

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
