class TreeNode {
    constructor(val = 0, left = null, right = null) {
        this.val = val;
        this.left = left;
        this.right = right;
    }
}

function lowestCommonAncestorRecursive(root, p, q) {
    if (root.val > p.val && root.val > q.val) {
        // Both p and q are in left subtree
        return lowestCommonAncestorRecursive(root.left, p, q);
    } else if (root.val < p.val && root.val < q.val) {
        // Both p and q are in right subtree
        return lowestCommonAncestorRecursive(root.right, p, q);
    } else {
        // p and q are on different sides or one of them is root
        return root;
    }
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
console.log(lowestCommonAncestorRecursive(root, p, q).val);  // 2
