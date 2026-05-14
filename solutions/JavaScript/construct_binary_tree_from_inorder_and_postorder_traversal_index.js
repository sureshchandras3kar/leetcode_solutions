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
 * Construct binary tree from inorder and postorder traversal using index tracking.
 *
 * Key insight:
 * - Postorder: left subtree, right subtree, root (last element is always root)
 * - Inorder: left subtree, root, right subtree
 * - Use a pointer to track the current root in postorder (traverse from end to start)
 * - Find root in inorder to split into left and right subtrees
 *
 * @param {number[]} inorder
 * @param {number[]} postorder
 * @return {TreeNode}
 *
 * Time Complexity: O(n²) in worst case due to linear search for root in inorder
 * Space Complexity: O(h) for recursion call stack where h is height
 */
var buildTree = function (inorder, postorder) {
    if (inorder.length === 0 || postorder.length === 0) {
        return null;
    }

    let post_idx = postorder.length - 1;

    const build = (in_start, in_end) => {
        /**
         * Recursively build tree by processing postorder from right to left.
         *
         * @param {number} in_start, in_end - Range in inorder array
         * @return {TreeNode}
         */

        if (in_start > in_end || post_idx < 0) {
            return null;
        }

        // Current postorder element (processing from end to start)
        const root_val = postorder[post_idx];
        post_idx--;

        const root = new TreeNode(root_val);

        // Find root position in inorder
        const root_idx = inorder.indexOf(root_val);

        // Build right subtree first (postorder: left, right, root)
        // Since we traverse postorder backwards, right comes before left
        root.right = build(root_idx + 1, in_end);

        // Build left subtree
        root.left = build(in_start, root_idx - 1);

        return root;
    };

    return build(0, inorder.length - 1);
};

// Helper function for inorder traversal
function inorder_traversal(node) {
    if (!node) return [];
    return [
        ...inorder_traversal(node.left),
        node.val,
        ...inorder_traversal(node.right),
    ];
}

// Helper function for postorder traversal
function postorder_traversal(node) {
    if (!node) return [];
    return [
        ...postorder_traversal(node.left),
        ...postorder_traversal(node.right),
        node.val,
    ];
}

// Test cases
if (require.main === module) {
    // Example 1: [3,9,20,null,null,15,7]
    //     3
    //    / \
    //   9  20
    //      / \
    //     15  7
    const inorder1 = [9, 3, 15, 20, 7];
    const postorder1 = [9, 15, 7, 20, 3];
    const root1 = buildTree(inorder1, postorder1);

    console.log("Inorder:", inorder_traversal(root1));   // Expected: [9, 3, 15, 20, 7]
    console.log("Postorder:", postorder_traversal(root1)); // Expected: [9, 15, 7, 20, 3]

    // Example 2: Single node
    const inorder2 = [1];
    const postorder2 = [1];
    const root2 = buildTree(inorder2, postorder2);
    console.log("Single node:", inorder_traversal(root2));

    // Example 3: Left skewed tree
    const inorder3 = [3, 2, 1];
    const postorder3 = [3, 2, 1];
    const root3 = buildTree(inorder3, postorder3);
    console.log("Left skewed:", inorder_traversal(root3));
}

module.exports = { buildTree, TreeNode };
