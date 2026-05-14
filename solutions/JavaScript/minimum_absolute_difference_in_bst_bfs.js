class TreeNode {
    constructor(val = 0, left = null, right = null) {
        this.val = val;
        this.left = left;
        this.right = right;
    }
}

function getMinimumDifferenceBFS(root) {
    const stack = [];
    let current = root;
    let prev = null;
    let minDiff = Infinity;

    while (stack.length > 0 || current) {
        // Go to leftmost node
        while (current) {
            stack.push(current);
            current = current.left;
        }

        // Current is null, pop from stack
        current = stack.pop();

        // Process current node
        if (prev !== null) {
            minDiff = Math.min(minDiff, current.val - prev);
        }
        prev = current.val;

        // Visit right subtree
        current = current.right;
    }

    return minDiff;
}

// Test cases
const root1 = new TreeNode(4);
root1.left = new TreeNode(2);
root1.right = new TreeNode(6);
root1.left.left = new TreeNode(1);
root1.left.right = new TreeNode(3);
console.log(getMinimumDifferenceBFS(root1));  // 1

const root2 = new TreeNode(1);
root2.right = new TreeNode(5);
root2.right.left = new TreeNode(4);
console.log(getMinimumDifferenceBFS(root2));  // 1
