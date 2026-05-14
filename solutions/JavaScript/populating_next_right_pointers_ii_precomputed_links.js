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
 * Populates next pointers using pre-computed links without queue.
 * Uses next pointers of parent level to traverse current level.
 * Time: O(n), Space: O(1)
 * @param {Node} root
 * @return {Node}
 */
function connect(root) {
    if (!root) return root;

    let leftmost = root;

    while (leftmost) {
        let prev = null;
        let current = leftmost;

        while (current) {
            if (current.left) {
                if (prev) {
                    prev.next = current.left;
                }
                prev = current.left;
            }

            if (current.right) {
                if (prev) {
                    prev.next = current.right;
                }
                prev = current.right;
            }

            current = current.next;
        }

        leftmost = leftmost.left;
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
console.log("Example 1: Tree with next pointers connected via pre-computed links");
