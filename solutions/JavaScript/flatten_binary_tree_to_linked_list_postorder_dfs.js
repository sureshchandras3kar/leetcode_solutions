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
 * Flatten binary tree to linked list using post-order DFS.
 * Uses previous pointer to track last visited node in reverse in-order.
 * Time: O(n), Space: O(h) for recursion stack
 * @param {TreeNode} root
 * @return {void}
 */
function flatten(root) {
    let prev = null;

    function dfs(node) {
        if (!node) return;

        // Post-order: right, left, then process node
        dfs(node.right);
        dfs(node.left);

        node.right = prev;
        node.left = null;
        prev = node;
    }

    dfs(root);
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
