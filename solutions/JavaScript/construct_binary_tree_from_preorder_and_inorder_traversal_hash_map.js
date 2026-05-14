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
 * Construct a binary tree from preorder and inorder traversal using hash map.
 * Time: O(n), Space: O(n)
 * @param {number[]} preorder
 * @param {number[]} inorder
 * @return {TreeNode}
 */
function buildTree(preorder, inorder) {
    if (preorder.length === 0 || inorder.length === 0) {
        return null;
    }

    // Create a map for quick lookup of indices in inorder
    const inorderMap = new Map();
    for (let i = 0; i < inorder.length; i++) {
        inorderMap.set(inorder[i], i);
    }

    const build = (preorderStart, preorderEnd, inorderStart, inorderEnd) => {
        if (preorderStart > preorderEnd || inorderStart > inorderEnd) {
            return null;
        }

        // Root is the first element in preorder
        const rootVal = preorder[preorderStart];
        const root = new TreeNode(rootVal);

        // Find root position in inorder
        const rootInorderIdx = inorderMap.get(rootVal);

        // Number of elements in left subtree
        const leftSize = rootInorderIdx - inorderStart;

        // Build left subtree
        root.left = build(
            preorderStart + 1,
            preorderStart + leftSize,
            inorderStart,
            rootInorderIdx - 1
        );

        // Build right subtree
        root.right = build(
            preorderStart + leftSize + 1,
            preorderEnd,
            rootInorderIdx + 1,
            inorderEnd
        );

        return root;
    };

    return build(0, preorder.length - 1, 0, inorder.length - 1);
}

// Test cases
if (require.main === module) {
    // Test 1: [3,9,20,15,7], [9,3,15,20,7]
    const preorder1 = [3, 9, 20, 15, 7];
    const inorder1 = [9, 3, 15, 20, 7];
    const root1 = buildTree(preorder1, inorder1);
    console.log(`Test 1 - Root: ${root1.val}`);  // 3

    // Test 2: [1], [1]
    const preorder2 = [1];
    const inorder2 = [1];
    const root2 = buildTree(preorder2, inorder2);
    console.log(`Test 2 - Root: ${root2.val}`);  // 1
}

module.exports = buildTree;
