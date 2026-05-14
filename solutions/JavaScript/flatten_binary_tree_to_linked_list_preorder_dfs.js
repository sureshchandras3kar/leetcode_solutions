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
 * Flatten binary tree to linked list using pre-order DFS.
 * Recursively flattens left and right subtrees, then rewires pointers.
 * Time: O(n), Space: O(h) for recursion stack
 * @param {TreeNode} root
 * @return {void}
 */
function flatten(root) {
    if (!root) return;

    flatten(root.left);
    flatten(root.right);

    if (root.left) {
        // Find rightmost node in flattened left subtree
        let rightmost = root.left;
        while (rightmost.right) {
            rightmost = rightmost.right;
        }

        // Attach right subtree to rightmost node
        rightmost.right = root.right;
        // Move flattened left subtree to right
        root.right = root.left;
        root.left = null;
    }
}

// Test cases
const root = new TreeNode(1);
root.left = new TreeNode(2);
root.left.left = new TreeNode(3);
root.left.right = new TreeNode(4);
root.right = new TreeNode(5);
root.right.right = new TreeNode(6);

flatten(root);

const result = [];
let current = root;
while (current) {
    result.push(current.val);
    current = current.right;
}
console.log("Example 1:", result);  // [1, 2, 3, 4, 5, 6]
