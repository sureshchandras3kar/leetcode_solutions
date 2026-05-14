function TreeNode(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
}

/**
 * Find the maximum depth of a binary tree using DFS (recursive).
 *
 * Time Complexity: O(n) where n is the number of nodes
 * Space Complexity: O(h) where h is the height (call stack depth)
 *
 * @param {TreeNode} root
 * @return {number}
 */
function maxDepth(root) {
    if (root === null) {
        return 0;
    }

    return 1 + Math.max(maxDepth(root.left), maxDepth(root.right));
}

// Example 1: [3,9,20,null,null,15,7]
//       3
//      / \
//     9  20
//       /  \
//      15   7
const root1 = new TreeNode(3);
root1.left = new TreeNode(9);
root1.right = new TreeNode(20);
root1.right.left = new TreeNode(15);
root1.right.right = new TreeNode(7);

console.log(maxDepth(root1)); // Expected: 3

// Example 2: [1,null,2]
//     1
//      \
//       2
const root2 = new TreeNode(1);
root2.right = new TreeNode(2);

console.log(maxDepth(root2)); // Expected: 2

// Example 3: Empty tree
const root3 = null;
console.log(maxDepth(root3)); // Expected: 0

export default maxDepth;
