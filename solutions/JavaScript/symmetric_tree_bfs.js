class TreeNode {
    constructor(val = 0, left = null, right = null) {
        this.val = val;
        this.left = left;
        this.right = right;
    }
}

function isSymmetricBFS(root) {
    /**
     * Check if a binary tree is symmetric using BFS with a queue.
     *
     * Time Complexity: O(n) - visit each node once
     * Space Complexity: O(w) - queue width, where w is max nodes at any level
     */
    if (!root) {
        return true;
    }

    const queue = [[root.left, root.right]];

    while (queue.length > 0) {
        const [left, right] = queue.shift();

        // Both nodes are null - continue (symmetric so far)
        if (!left && !right) {
            continue;
        }

        // One node is null or values differ - not symmetric
        if (!left || !right) {
            return false;
        }
        if (left.val !== right.val) {
            return false;
        }

        // Add pairs for next level: left's left with right's right
        // and left's right with right's left (mirror pattern)
        queue.push([left.left, right.right]);
        queue.push([left.right, right.left]);
    }

    return true;
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
    console.log(isSymmetricBFS(root1)); // true

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
    console.log(isSymmetricBFS(root2)); // false

    // Example 3: Single node
    const root3 = new TreeNode(1);
    console.log(isSymmetricBFS(root3)); // true
}

module.exports = isSymmetricBFS;
