function TreeNode(val, left = null, right = null) { this.val = val; this.left = left; this.right = right; }

function levelOrder(root) {
    const result = []; if (!root) return result;
    const queue = [root];
    while (queue.length) {
        const sz = queue.length, level = [];
        for (let i = 0; i < sz; i++) {
            const n = queue.shift();
            level.push(n.val);
            if (n.left) queue.push(n.left);
            if (n.right) queue.push(n.right);
        }
        result.push(level);
    }
    return result;
}

module.exports = levelOrder;
