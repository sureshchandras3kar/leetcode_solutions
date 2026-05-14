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
 * Binary Search Tree Iterator using stack for in-order traversal.
 * Implements lazy evaluation: next() O(1) amortized, hasNext() O(1).
 * Space: O(h) where h is height
 */
class BSTIterator {
    constructor(root) {
        this.stack = [];
        this.pushLeft(root);
    }

    pushLeft(node) {
        /**Push all left nodes onto stack.*/
        while (node) {
            this.stack.push(node);
            node = node.left;
        }
    }

    /**
     * Return next smallest element.
     * Time: O(1) amortized
     */
    next() {
        const node = this.stack.pop();

        if (node.right) {
            this.pushLeft(node.right);
        }

        return node.val;
    }

    /**
     * Check if there are more elements.
     * Time: O(1)
     */
    hasNext() {
        return this.stack.length > 0;
    }
}

// Test cases
const root = new TreeNode(7);
root.left = new TreeNode(3);
root.right = new TreeNode(15);
root.right.left = new TreeNode(9);
root.right.right = new TreeNode(20);

const iterator = new BSTIterator(root);
const result = [];
while (iterator.hasNext()) {
    result.push(iterator.next());
}
console.log("In-order traversal:", result);  // [3, 7, 9, 15, 20]
