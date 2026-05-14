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
 * Construct a binary tree from preorder and inorder traversal using index tracking.
 * Time: O(n), Space: O(h) where h is height
 * @param {number[]} preorder
 * @param {number[]} inorder
 * @return {TreeNode}
 */
function buildTree(preorder, inorder) {
    if (preorder.length === 0 || inorder.length === 0) {
        return null;
    }

    let preorderIdx = 0;

    const build = (inorderStart, inorderEnd) => {
        if (inorderStart > inorderEnd || preorderIdx >= preorder.length) {
            return null;
        }

        // Root is the current element in preorder
        const rootVal = preorder[preorderIdx];
        preorderIdx++;
        const root = new TreeNode(rootVal);

        // Find root position in inorder
        let rootInorderIdx = -1;
        for (let i = inorderStart; i <= inorderEnd; i++) {
            if (inorder[i] === rootVal) {
                rootInorderIdx = i;
                break;
            }
        }

        // Build left subtree
        root.left = build(inorderStart, rootInorderIdx - 1);

        // Build right subtree
        root.right = build(rootInorderIdx + 1, inorderEnd);

        return root;
    };

    return build(0, inorder.length - 1);
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
