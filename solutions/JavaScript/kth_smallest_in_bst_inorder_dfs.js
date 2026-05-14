class TreeNode {
    constructor(val = 0, left = null, right = null) {
        this.val = val;
        this.left = left;
        this.right = right;
    }
}

function kthSmallestInorderDFS(root, k) {
    let count = 0;
    let result = null;

    function inorder(node) {
        if (!node || result !== null) return;

        // Traverse left subtree
        inorder(node.left);

        // Process current node
        count++;
        if (count === k) {
            result = node.val;
            return;
        }

        // Traverse right subtree
        inorder(node.right);
    }

    inorder(root);
    return result;
}

// Test cases
const root = new TreeNode(3);
root.left = new TreeNode(1);
root.right = new TreeNode(4);
root.left.right = new TreeNode(2);

console.log(kthSmallestInorderDFS(root, 1));  // 1
console.log(kthSmallestInorderDFS(root, 3));  // 2
