class TreeNode {
    constructor(val = 0, left = null, right = null) {
        this.val = val;
        this.left = left;
        this.right = right;
    }
}

function kthSmallestMorrisInorder(root, k) {
    let count = 0;
    let current = root;

    while (current) {
        if (!current.left) {
            // Left is null, process current node
            count++;
            if (count === k) {
                return current.val;
            }
            current = current.right;
        } else {
            // Find in-order predecessor
            let predecessor = current.left;
            while (predecessor.right && predecessor.right !== current) {
                predecessor = predecessor.right;
            }

            if (!predecessor.right) {
                // Create link to current node
                predecessor.right = current;
                current = current.left;
            } else {
                // Link exists, remove it and process current
                predecessor.right = null;
                count++;
                if (count === k) {
                    return current.val;
                }
                current = current.right;
            }
        }
    }

    return -1;
}

// Test cases
const root = new TreeNode(3);
root.left = new TreeNode(1);
root.right = new TreeNode(4);
root.left.right = new TreeNode(2);

console.log(kthSmallestMorrisInorder(root, 1));  // 1
console.log(kthSmallestMorrisInorder(root, 3));  // 2
