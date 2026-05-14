#include <iostream>
#include <memory>

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
     * DFS Recursive approach to invert a binary tree.
     * Recursively swap left and right children for each node.
     *
     * Time Complexity: O(n) - visit each node once
     * Space Complexity: O(h) - recursion stack depth (h = height)
     */
    TreeNode* invertTree(TreeNode* root) {
        if (root == nullptr) {
            return nullptr;
        }

        // Swap left and right children
        swap(root->left, root->right);

        // Recursively invert left and right subtrees
        invertTree(root->left);
        invertTree(root->right);

        return root;
    }
};

int main() {
    // Create tree:     1
    //                /   \
    //               2     3
    TreeNode* root = new TreeNode(1);
    root->left = new TreeNode(2);
    root->right = new TreeNode(3);

    Solution sol;
    sol.invertTree(root);

    // Expected:        1
    //                /   \
    //               3     2
    cout << "Root: " << root->val << endl;  // 1
    cout << "Left: " << root->left->val << ", Right: " << root->right->val << endl;  // 3, 2

    return 0;
}
