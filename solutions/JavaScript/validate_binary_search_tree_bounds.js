function TreeNode(val, left = null, right = null) { this.val = val; this.left = left; this.right = right; }

function isValidBST(root) {
    function dfs(n, minVal, maxVal) {
        if (!n) return true;
        if (n.val <= minVal || n.val >= maxVal) return false;
        return dfs(n.left, minVal, n.val) && dfs(n.right, n.val, maxVal);
    }
    return dfs(root, Number.NEGATIVE_INFINITY, Number.POSITIVE_INFINITY);
}
module.exports = isValidBST;
