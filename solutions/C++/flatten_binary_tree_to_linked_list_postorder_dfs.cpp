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
private:
    TreeNode* prev = nullptr;

    void dfs(TreeNode* node) {
        if (!node) return;

        // Post-order: right, left, then process node
        dfs(node->right);
        dfs(node->left);

        node->right = prev;
        node->left = nullptr;
        prev = node;
    }

public:
    void flatten(TreeNode* root) {
        /**
         * Flatten binary tree to linked list using post-order DFS.
         * Uses previous pointer to track last visited node in reverse in-order.
         * Time: O(n), Space: O(h) for recursion stack
         */
        dfs(root);
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

    cout << "Example 1: Tree flattened to linked list using post-order DFS" << endl;

    return 0;
}
