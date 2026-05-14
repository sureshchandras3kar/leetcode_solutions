function TreeNode(val, left = null, right = null) { this.val = val; this.left = left; this.right = right; }

function isValidBST(root) {
    let prev = Number.NEGATIVE_INFINITY;
    function dfs(n) {
        if (!n) return true;
        if (!dfs(n.left)) return false;
        if (n.val <= prev) return false;
        prev = n.val;
        return dfs(n.right);
    }
    return dfs(root);
}
module.exports = isValidBST;
