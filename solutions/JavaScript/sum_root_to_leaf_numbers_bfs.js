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
 * Sum all root-to-leaf numbers using iterative BFS.
 * Queue stores [node, current_number] pairs.
 * Time: O(n), Space: O(w) where w is max width
 * @param {TreeNode} root
 * @return {number}
 */
function sumNumbers(root) {
    if (!root) return 0;

    const queue = [[root, root.val]];
    let total = 0;

    while (queue.length > 0) {
        const [node, currentSum] = queue.shift();

        // Leaf node: add to total
        if (!node.left && !node.right) {
            total += currentSum;
            continue;
        }

        if (node.left) {
            queue.push([node.left, currentSum * 10 + node.left.val]);
        }
        if (node.right) {
            queue.push([node.right, currentSum * 10 + node.right.val]);
        }
    }

    return total;
}

// Test cases
const root1 = new TreeNode(1);
root1.left = new TreeNode(2);
root1.right = new TreeNode(3);

console.log("Example 1 sum:", sumNumbers(root1));  // 25 (12 + 13)

const root2 = new TreeNode(4);
root2.left = new TreeNode(9);
root2.right = new TreeNode(0);
root2.left.left = new TreeNode(5);
root2.left.right = new TreeNode(1);

console.log("Example 2 sum:", sumNumbers(root2));  // 1026 (495 + 491 + 40)
