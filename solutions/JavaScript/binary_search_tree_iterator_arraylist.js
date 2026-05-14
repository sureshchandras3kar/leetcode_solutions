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
 * Binary Search Tree Iterator using pre-computed array.
 * Stores all in-order elements upfront.
 * Space: O(n), next() O(1), hasNext() O(1)
 */
class BSTIterator {
    constructor(root) {
        this.arr = [];
        this.index = 0;
        this.inorder(root);
    }

    inorder(node) {
        /**Pre-compute in-order traversal into array.*/
        if (!node) return;

        this.inorder(node.left);
        this.arr.push(node.val);
        this.inorder(node.right);
    }

    /**
     * Return next smallest element.
     * Time: O(1)
     */
    next() {
        return this.arr[this.index++];
    }

    /**
     * Check if there are more elements.
     * Time: O(1)
     */
    hasNext() {
        return this.index < this.arr.length;
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
