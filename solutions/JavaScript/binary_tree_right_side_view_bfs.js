function TreeNode(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
}

function rightSideView(root) {
    const result = [];
    if (!root) return result;

    const queue = [root];

    while (queue.length > 0) {
        const levelSize = queue.length;

        for (let i = 0; i < levelSize; i++) {
            const node = queue.shift();

            if (i === levelSize - 1) {
                result.push(node.val);
            }

            if (node.left) queue.push(node.left);
            if (node.right) queue.push(node.right);
        }
    }

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
