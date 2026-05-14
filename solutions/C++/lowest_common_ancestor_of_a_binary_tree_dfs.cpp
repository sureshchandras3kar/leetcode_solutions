#include <iostream>
using namespace std;

struct TreeNode {
    int val;
    TreeNode* left;
    TreeNode* right;
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
};

class Solution {
public:
    /**
     * Find the lowest common ancestor using DFS recursion.
     *
     * Time Complexity: O(n) where n is the number of nodes
     * Space Complexity: O(h) where h is the height (call stack depth)
     */
    TreeNode* lowestCommonAncestor(TreeNode* root, TreeNode* p, TreeNode* q) {
        if (root == nullptr) {
            return nullptr;
        }

        // If either p or q is the current node, it's a potential LCA
        if (root == p || root == q) {
            return root;
        }

        // Search in left and right subtrees
        TreeNode* left = lowestCommonAncestor(root->left, p, q);
        TreeNode* right = lowestCommonAncestor(root->right, p, q);

        // If both p and q are found in different subtrees, root is LCA
        if (left != nullptr && right != nullptr) {
            return root;
        }

        // If both are in one subtree, return that subtree's result
        return left != nullptr ? left : right;
    }
};

int main() {
    Solution solution;

    // Example 1: [3,5,1,6,2,0,8,null,null,7,4]
    //       3
    //      / \
    //     5   1
    //    / \ / \
    //   6  2 0  8
    //     / \
    //    7   4
    TreeNode* root1 = new TreeNode(3);
    root1->left = new TreeNode(5);
    root1->right = new TreeNode(1);
    root1->left->left = new TreeNode(6);
    root1->left->right = new TreeNode(2);
    root1->right->left = new TreeNode(0);
    root1->right->right = new TreeNode(8);
    root1->left->right->left = new TreeNode(7);
    root1->left->right->right = new TreeNode(4);

    TreeNode* p1 = root1->left;  // Node 5
    TreeNode* q1 = root1->right;  // Node 1
    TreeNode* result1 = solution.lowestCommonAncestor(root1, p1, q1);
    cout << "LCA of " << p1->val << " and " << q1->val << ": " << result1->val << endl;  // Expected: 3

    // Example 2: Same tree, p=5, q=4
    TreeNode* p2 = root1->left;  // Node 5
    TreeNode* q2 = root1->left->right->right;  // Node 4
    TreeNode* result2 = solution.lowestCommonAncestor(root1, p2, q2);
    cout << "LCA of " << p2->val << " and " << q2->val << ": " << result2->val << endl;  // Expected: 5

    // Example 3: [1,2]
    //     1
    //      \
    //       2
    TreeNode* root3 = new TreeNode(1);
    root3->right = new TreeNode(2);

    TreeNode* p3 = root3;  // Node 1
    TreeNode* q3 = root3->right;  // Node 2
    TreeNode* result3 = solution.lowestCommonAncestor(root3, p3, q3);
    cout << "LCA of " << p3->val << " and " << q3->val << ": " << result3->val << endl;  // Expected: 1

    return 0;
}
