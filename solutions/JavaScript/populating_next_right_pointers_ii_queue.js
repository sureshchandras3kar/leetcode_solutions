/**
 * Definition for a Node.
 */
class Node {
    constructor(val = 0, left = null, right = null, next = null) {
        this.val = val;
        this.left = left;
        this.right = right;
        this.next = next;
    }
}

/**
 * Populates next pointers using level-order BFS queue.
 * Time: O(n), Space: O(w) where w is max width
 * @param {Node} root
 * @return {Node}
 */
function connect(root) {
    if (!root) return root;

    const queue = [root];

    while (queue.length > 0) {
        const levelSize = queue.length;
        let prev = null;

        for (let i = 0; i < levelSize; i++) {
            const node = queue.shift();

            if (prev) {
                prev.next = node;
            }
            prev = node;

            if (node.left) queue.push(node.left);
            if (node.right) queue.push(node.right);
        }
    }

    return root;
}

// Test cases
const root = new Node(1);
root.left = new Node(2);
root.right = new Node(3);
root.left.left = new Node(4);
root.left.right = new Node(5);
root.right.right = new Node(7);

connect(root);
console.log("Example 1: Tree with next pointers connected via queue");
