function TreeNode(val, left = null, right = null) { this.val = val; this.left = left; this.right = right; }

function zigzagLevelOrder(root) {
    const result = [];
    function dfs(n, level) {
        if (!n) return;
        if (level === result.length) result.push([]);
        if (level % 2 === 0) result[level].push(n.val);
        else result[level].unshift(n.val);
        dfs(n.left, level + 1);
        dfs(n.right, level + 1);
    }
    dfs(root, 0);
    return result;
}
module.exports = zigzagLevelOrder;
