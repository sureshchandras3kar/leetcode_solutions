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
    int countNodes(TreeNode* root) {
        /**
         * Count nodes in complete binary tree using level calculation.
         * For complete tree, if left height == right height, left is perfect.
         * Time: O(log² n), Space: O(log n) for recursion
         */
        if (!root) return 0;

        // Calculate left and right heights
        int left_height = 0;
        TreeNode* left_node = root->left;
        while (left_node) {
            left_height++;
            left_node = left_node->left;
        }

        int right_height = 0;
        TreeNode* right_node = root->right;
        while (right_node) {
            right_height++;
            right_node = right_node->right;
        }

        if (left_height == right_height) {
            // Left subtree is perfect: 2^(h+1) - 1 nodes + root + recursively count right
            return (1 << (left_height + 1)) - 1 + countNodes(root->right);
        } else {
            // Right subtree is perfect: 2^h - 1 nodes + root + recursively count left
            return (1 << (right_height + 1)) - 1 + countNodes(root->left);
        }
    }
};

// Test cases
int main() {
    // Example 1: Complete tree with 5 nodes
    TreeNode* root = new TreeNode(1);
    root->left = new TreeNode(2);
    root->right = new TreeNode(3);
    root->left->left = new TreeNode(4);
    root->left->right = new TreeNode(5);

    Solution sol;
    cout << "Example 1 node count: " << sol.countNodes(root) << endl;  // 5

    // Example 2: Single node
    TreeNode* single = new TreeNode(1);
    cout << "Example 2 node count: " << sol.countNodes(single) << endl;  // 1

    // Example 3: Empty tree
    cout << "Example 3 node count: " << sol.countNodes(nullptr) << endl;  // 0

    return 0;
}
