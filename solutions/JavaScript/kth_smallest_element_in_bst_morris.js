class TreeNode {
    constructor(val = 0, left = null, right = null) {
        this.val = val;
        this.left = left;
        this.right = right;
    }
}

function kthSmallest_morris(root, k) {
    let count = 0;
    let current = root;

    while (current) {
        if (!current.left) {
            // No left child: visit current
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
                // First visit: create thread
                predecessor.right = current;
                current = current.left;
            } else {
                // Second visit: remove thread, process current
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

// Test
const root = new TreeNode(3);
root.left = new TreeNode(1);
root.right = new TreeNode(4);
root.left.right = new TreeNode(2);

console.log(kthSmallest_morris(root, 1));  // 1
console.log(kthSmallest_morris(root, 3));  // 2
