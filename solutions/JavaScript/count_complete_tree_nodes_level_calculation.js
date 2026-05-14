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
 * Count nodes in complete binary tree using level calculation.
 * For complete tree, if left height == right height, left is perfect.
 * Time: O(log² n), Space: O(log n) for recursion
 * @param {TreeNode} root
 * @return {number}
 */
function countNodes(root) {
    if (!root) return 0;

    // Calculate left and right heights
    let leftHeight = 0;
    let leftNode = root.left;
    while (leftNode) {
        leftHeight++;
        leftNode = leftNode.left;
    }

    let rightHeight = 0;
    let rightNode = root.right;
    while (rightNode) {
        rightHeight++;
        rightNode = rightNode.right;
    }

    if (leftHeight === rightHeight) {
        // Left subtree is perfect: 2^(h+1) - 1 nodes + root + recursively count right
        return (1 << (leftHeight + 1)) - 1 + countNodes(root.right);
    } else {
        // Right subtree is perfect: 2^h - 1 nodes + root + recursively count left
        return (1 << (rightHeight + 1)) - 1 + countNodes(root.left);
    }
}

// Test cases
const root1 = new TreeNode(1);
root1.left = new TreeNode(2);
root1.right = new TreeNode(3);
root1.left.left = new TreeNode(4);
root1.left.right = new TreeNode(5);

console.log("Example 1 node count:", countNodes(root1));  // 5

const single = new TreeNode(1);
console.log("Example 2 node count:", countNodes(single));  // 1

console.log("Example 3 node count:", countNodes(null));  // 0
