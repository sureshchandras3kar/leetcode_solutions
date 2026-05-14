function TreeNode(val, left = null, right = null) { this.val = val; this.left = left; this.right = right; }

function levelOrder(root) {
    const result = [];
    function dfs(n, level) {
        if (!n) return;
        if (level === result.length) result.push([]);
        result[level].push(n.val);
        dfs(n.left, level + 1);
        dfs(n.right, level + 1);
    }
    dfs(root, 0);
    return result;
}

module.exports = levelOrder;
