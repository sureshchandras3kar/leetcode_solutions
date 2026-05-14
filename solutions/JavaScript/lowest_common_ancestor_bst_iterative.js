class TreeNode {
    constructor(val = 0, left = null, right = null) {
        this.val = val;
        this.left = left;
        this.right = right;
    }
}

function lowestCommonAncestorIterative(root, p, q) {
    let current = root;

    while (current) {
        if (current.val > p.val && current.val > q.val) {
            // Both p and q are in left subtree
            current = current.left;
        } else if (current.val < p.val && current.val < q.val) {
            // Both p and q are in right subtree
            current = current.right;
        } else {
            // p and q are on different sides or one of them is current
            return current;
        }
    }

    return root;
}

// Test cases
const root = new TreeNode(6);
root.left = new TreeNode(2);
root.right = new TreeNode(8);
root.left.left = new TreeNode(0);
root.left.right = new TreeNode(4);
root.left.right.left = new TreeNode(3);
root.left.right.right = new TreeNode(5);
root.right.left = new TreeNode(7);
root.right.right = new TreeNode(9);

const p = root.left;
const q = root.left.right;
console.log(lowestCommonAncestorIterative(root, p, q).val);  // 2
