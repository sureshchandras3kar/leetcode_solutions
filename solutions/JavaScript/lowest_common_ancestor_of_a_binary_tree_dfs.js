/**
 * Definition for a binary tree node.
 */
function TreeNode(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
}

/**
 * Find the lowest common ancestor using DFS recursion.
 *
 * Time Complexity: O(n) where n is the number of nodes
 * Space Complexity: O(h) where h is the height (call stack depth)
 *
 * @param {TreeNode} root
 * @param {TreeNode} p
 * @param {TreeNode} q
 * @return {TreeNode}
 */
function lowestCommonAncestor(root, p, q) {
    if (root === null) {
        return null;
    }

    // If either p or q is the current node, it's a potential LCA
    if (root === p || root === q) {
        return root;
    }

    // Search in left and right subtrees
    const left = lowestCommonAncestor(root.left, p, q);
    const right = lowestCommonAncestor(root.right, p, q);

    // If both p and q are found in different subtrees, root is LCA
    if (left !== null && right !== null) {
        return root;
    }

    // If both are in one subtree, return that subtree's result
    return left !== null ? left : right;
}

// Test cases
if (require.main === module) {
    // Example 1: [3,5,1,6,2,0,8,null,null,7,4]
    //       3
    //      / \
    //     5   1
    //    / \ / \
    //   6  2 0  8
    //     / \
    //    7   4
    const root1 = new TreeNode(3);
    root1.left = new TreeNode(5);
    root1.right = new TreeNode(1);
    root1.left.left = new TreeNode(6);
    root1.left.right = new TreeNode(2);
    root1.right.left = new TreeNode(0);
    root1.right.right = new TreeNode(8);
    root1.left.right.left = new TreeNode(7);
    root1.left.right.right = new TreeNode(4);

    const p1 = root1.left;  // Node 5
    const q1 = root1.right;  // Node 1
    const result1 = lowestCommonAncestor(root1, p1, q1);
    console.log(`LCA of ${p1.val} and ${q1.val}: ${result1.val}`);  // Expected: 3

    // Example 2: Same tree, p=5, q=4
    const p2 = root1.left;  // Node 5
    const q2 = root1.left.right.right;  // Node 4
    const result2 = lowestCommonAncestor(root1, p2, q2);
    console.log(`LCA of ${p2.val} and ${q2.val}: ${result2.val}`);  // Expected: 5

    // Example 3: [1,2]
    //     1
    //      \
    //       2
    const root3 = new TreeNode(1);
    root3.right = new TreeNode(2);

    const p3 = root3;  // Node 1
    const q3 = root3.right;  // Node 2
    const result3 = lowestCommonAncestor(root3, p3, q3);
    console.log(`LCA of ${p3.val} and ${q3.val}: ${result3.val}`);  // Expected: 1
}

module.exports = lowestCommonAncestor;
