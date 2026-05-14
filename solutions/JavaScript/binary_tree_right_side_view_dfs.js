function TreeNode(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
}

function rightSideView(root) {
    const result = [];

    function dfs(node, level) {
        if (!node) return;

        if (level === result.length) {
            result.push(node.val);
        }

        dfs(node.right, level + 1);
        dfs(node.left, level + 1);
    }

    dfs(root, 0);
    return result;
}

if (require.main === module) {
    const root1 = new TreeNode(1);
    root1.left = new TreeNode(2);
    root1.right = new TreeNode(3);
    root1.left.right = new TreeNode(5);
    root1.right.right = new TreeNode(4);

    const result1 = rightSideView(root1);
    console.log("Right side view:", result1);  // Expected: [1, 3, 4]
}

module.exports = rightSideView;
