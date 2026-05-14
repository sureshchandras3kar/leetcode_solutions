class TreeNode {
    constructor(val = 0, left = null, right = null) {
        this.val = val;
        this.left = left;
        this.right = right;
    }
}

function getMinimumDifferenceInorderDFS(root) {
    let minDiff = Infinity;
    let prev = null;

    function inorder(node) {
        if (!node) return;

        // Traverse left subtree
        inorder(node.left);

        // Process current node
        if (prev !== null) {
            minDiff = Math.min(minDiff, node.val - prev);
        }
        prev = node.val;

        // Traverse right subtree
        inorder(node.right);
    }

    inorder(root);
    return minDiff;
}

// Test cases
const root1 = new TreeNode(4);
root1.left = new TreeNode(2);
root1.right = new TreeNode(6);
root1.left.left = new TreeNode(1);
root1.left.right = new TreeNode(3);
console.log(getMinimumDifferenceInorderDFS(root1));  // 1

const root2 = new TreeNode(1);
root2.right = new TreeNode(5);
root2.right.left = new TreeNode(4);
console.log(getMinimumDifferenceInorderDFS(root2));  // 1
