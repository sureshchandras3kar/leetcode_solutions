/**
 * Definition for a binary tree node.
 */
class TreeNode {
    constructor(val = 0, left = null, right = null) {
        this.val = val;
        this.left = left;
        this.right = right;
    }
}

/**
 * Count nodes in complete binary tree using binary search on node positions.
 * Uses existence check for node at each possible position.
 * Time: O(log² n), Space: O(log n) for recursion
 * @param {TreeNode} root
 * @return {number}
 */
function countNodes(root) {
    if (!root) return 0;

    function exists(pos, height, node) {
        /**Check if node at position pos exists.*/
        let left = 0n, right = (1n << BigInt(height - 1)) - 1n;

        for (let i = 0; i < height - 1; i++) {
            const mid = (left + right + 1n) / 2n;
            if (pos >= mid) {
                node = node.right;
                left = mid;
            } else {
                node = node.left;
                right = mid - 1n;
            }
        }

        return node !== null;
    }

    // Find height of tree
    let height = 0;
    let node = root;
    while (node) {
        height++;
        node = node.left;
    }

    // Binary search on number of nodes
    let low = 1n << BigInt(height - 1);    // 2^(h-1)
    let high = (1n << BigInt(height)) - 1n;  // 2^h - 1

    while (low <= high) {
        const mid = (low + high + 1n) / 2n;
        if (exists(mid, height, root)) {
            low = mid;
        } else {
            high = mid - 1n;
        }
    }

    return Number(low);
}

// Test cases
const root1 = new TreeNode(1);
root1.left = new TreeNode(2);
root1.right = new TreeNode(3);
root1.left.left = new TreeNode(4);
root1.left.right = new TreeNode(5);

console.log("Example 1 node count:", countNodes(root1));  // 5

const single = new TreeNode(1);
console.log("Example 2 node count:", countNodes(single));  // 1

console.log("Example 3 node count:", countNodes(null));  // 0
