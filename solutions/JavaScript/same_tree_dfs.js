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
 * Check if two binary trees are the same using DFS (recursive).
 *
 * Time Complexity: O(min(m, n)) where m and n are the number of nodes
 * Space Complexity: O(min(h1, h2)) where h1 and h2 are the heights (call stack)
 *
 * @param {TreeNode} p - First binary tree
 * @param {TreeNode} q - Second binary tree
 * @return {boolean} - True if the trees are the same, false otherwise
 */
function isSameTree(p, q) {
    // Both nodes are null (base case: equal)
    if (p === null && q === null) {
        return true;
    }

    // One is null, the other isn't (not equal)
    if (p === null || q === null) {
        return false;
    }

    // Values differ (not equal)
    if (p.val !== q.val) {
        return false;
    }

    // Recursively check left and right subtrees
    return isSameTree(p.left, q.left) && isSameTree(p.right, q.right);
}

// Test cases
if (require.main === module) {
    // Example 1: Same trees
    //     1           1
    //    / \         / \
    //   2   3       2   3
    const p1 = new TreeNode(1);
    p1.left = new TreeNode(2);
    p1.right = new TreeNode(3);

    const q1 = new TreeNode(1);
    q1.left = new TreeNode(2);
    q1.right = new TreeNode(3);

    console.log(isSameTree(p1, q1)); // Expected: true

    // Example 2: Different structure
    //     1           1
    //    /             \
    //   2               2
    const p2 = new TreeNode(1);
    p2.left = new TreeNode(2);

    const q2 = new TreeNode(1);
    q2.right = new TreeNode(2);

    console.log(isSameTree(p2, q2)); // Expected: false

    // Example 3: Different values
    //     1           1
    //    / \         / \
    //   2   1       1   2
    const p3 = new TreeNode(1);
    p3.left = new TreeNode(2);
    p3.right = new TreeNode(1);

    const q3 = new TreeNode(1);
    q3.left = new TreeNode(1);
    q3.right = new TreeNode(2);

    console.log(isSameTree(p3, q3)); // Expected: false

    // Example 4: Both empty
    console.log(isSameTree(null, null)); // Expected: true
}

module.exports = isSameTree;
