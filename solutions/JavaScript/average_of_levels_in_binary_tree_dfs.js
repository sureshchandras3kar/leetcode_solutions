function TreeNode(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
}

function averageOfLevels(root) {
    const sums = [];
    const counts = [];

    function dfs(node, level) {
        if (!node) return;

        if (level === sums.length) {
            sums.push(0);
            counts.push(0);
        }

        sums[level] += node.val;
        counts[level]++;

        dfs(node.left, level + 1);
        dfs(node.right, level + 1);
    }

    dfs(root, 0);

    return sums.map((sum, i) => sum / counts[i]);
}

if (require.main === module) {
    const root = new TreeNode(3);
    root.left = new TreeNode(9);
    root.right = new TreeNode(20);
    root.right.left = new TreeNode(15);
    root.right.right = new TreeNode(7);

    const result = averageOfLevels(root);
    console.log("Averages:", result);
}

module.exports = averageOfLevels;
