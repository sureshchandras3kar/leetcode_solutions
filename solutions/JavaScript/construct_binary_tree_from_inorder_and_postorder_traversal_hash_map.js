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
 * Construct binary tree from inorder and postorder traversal using HashMap.
 *
 * Key insight:
 * - Postorder: left subtree, right subtree, root (last element is always root)
 * - Inorder: left subtree, root, right subtree
 *
 * Use a HashMap to quickly find the root's position in inorder,
 * then recursively build left and right subtrees.
 *
 * @param {number[]} inorder
 * @param {number[]} postorder
 * @return {TreeNode}
 *
 * Time Complexity: O(n) where n is the number of nodes
 * Space Complexity: O(n) for HashMap and recursion call stack
 */
var buildTree = function (inorder, postorder) {
    if (inorder.length === 0 || postorder.length === 0) {
        return null;
    }

    // Build HashMap for O(1) inorder lookup
    const inorder_map = new Map();
    for (let i = 0; i < inorder.length; i++) {
        inorder_map.set(inorder[i], i);
    }

    const build = (post_start, post_end, in_start, in_end) => {
        /**
         * Recursively build tree from postorder and inorder ranges.
         *
         * @param {number} post_start, post_end - Range in postorder array
         * @param {number} in_start, in_end - Range in inorder array
         * @return {TreeNode}
         */

        if (post_start > post_end || in_start > in_end) {
            return null;
        }

        // Last element in postorder range is the root
        const root_val = postorder[post_end];
        const root = new TreeNode(root_val);

        // Find root position in inorder
        const root_idx = inorder_map.get(root_val);

        // Number of nodes in left subtree
        const left_size = root_idx - in_start;

        // Recursively build left subtree
        root.left = build(
            post_start,
            post_start + left_size - 1,
            in_start,
            root_idx - 1
        );

        // Recursively build right subtree
        root.right = build(
            post_start + left_size,
            post_end - 1,
            root_idx + 1,
            in_end
        );

        return root;
    };

    return build(0, postorder.length - 1, 0, inorder.length - 1);
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
