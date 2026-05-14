class TreeNode {
    constructor(val = 0, left = null, right = null) {
        this.val = val;
        this.left = left;
        this.right = right;
    }
}

function getMinimumDifference_inorder(root) {
    let minDiff = Infinity;
    let prev = null;

    function inorder(node) {
        if (!node) return;

        inorder(node.left);

        if (prev !== null) {
            minDiff = Math.min(minDiff, node.val - prev);
        }
        prev = node.val;

        inorder(node.right);
    }

    inorder(root);
    return minDiff;
}

// Test
const root = new TreeNode(4);
root.left = new TreeNode(2);
root.right = new TreeNode(6);
root.left.left = new TreeNode(1);
root.left.right = new TreeNode(3);

console.log(getMinimumDifference_inorder(root));  // 1
