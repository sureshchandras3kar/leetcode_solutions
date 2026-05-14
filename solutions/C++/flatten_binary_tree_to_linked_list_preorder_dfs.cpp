#include <iostream>
using namespace std;

// Definition for a binary tree node.
struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
};

class Solution {
public:
    void flatten(TreeNode* root) {
        /**
         * Flatten binary tree to linked list using pre-order DFS.
         * Recursively flattens left and right subtrees, then rewires pointers.
         * Time: O(n), Space: O(h) for recursion stack
         */
        if (!root) return;

        flatten(root->left);
        flatten(root->right);

        if (root->left) {
            // Find rightmost node in flattened left subtree
            TreeNode* rightmost = root->left;
            while (rightmost->right) {
                rightmost = rightmost->right;
            }

            // Attach right subtree to rightmost node
            rightmost->right = root->right;
            // Move flattened left subtree to right
            root->right = root->left;
            root->left = nullptr;
        }
    }
};

// Test cases
int main() {
    // Example 1: [1,2,5,3,4,null,6]
    TreeNode* root = new TreeNode(1);
    root->left = new TreeNode(2);
    root->left->left = new TreeNode(3);
    root->left->right = new TreeNode(4);
    root->right = new TreeNode(5);
    root->right->right = new TreeNode(6);

    Solution sol;
    sol.flatten(root);

    cout << "Example 1: Tree flattened to linked list using pre-order DFS" << endl;

    return 0;
}
