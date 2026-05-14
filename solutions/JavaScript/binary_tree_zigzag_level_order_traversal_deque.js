function TreeNode(val, left = null, right = null) { this.val = val; this.left = left; this.right = right; }

function zigzagLevelOrder(root) {
    const result = []; if (!root) return result;
    const q = [root]; let level = 0;
    while (q.length) {
        const sz = q.length, dq = [];
        for (let i = 0; i < sz; i++) {
            const n = q.shift();
            if (level % 2 === 0) dq.push(n.val);
            else dq.unshift(n.val);
            if (n.left) q.push(n.left);
            if (n.right) q.push(n.right);
        }
        result.push(dq);
        level++;
    }
    return result;
}
module.exports = zigzagLevelOrder;
