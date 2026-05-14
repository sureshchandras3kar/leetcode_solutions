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
    bool hasPathSum(TreeNode* root, int targetSum) {
        /**
         * Check if tree has root-to-leaf path summing to targetSum using recursive DFS.
         * Time: O(n), Space: O(h) for recursion stack
         */
        if (!root) return false;

        // Leaf node check
        if (!root->left && !root->right) {
            return root->val == targetSum;
        }

        // Subtract current value and check left and right subtrees
        int remaining = targetSum - root->val;
        return hasPathSum(root->left, remaining) || hasPathSum(root->right, remaining);
    }
};

// Test cases
int main() {
    // Example 1: [5,4,8,11,null,13,4,7,2,null,1], targetSum = 22
    TreeNode* root = new TreeNode(5);
    root->left = new TreeNode(4);
    root->left->left = new TreeNode(11);
    root->left->left->left = new TreeNode(7);
    root->left->left->right = new TreeNode(2);
    root->right = new TreeNode(8);
    root->right->left = new TreeNode(13);
    root->right->right = new TreeNode(4);
    root->right->right->right = new TreeNode(1);

    Solution sol;
    cout << boolalpha;
    cout << "Example 1 (target 22): " << sol.hasPathSum(root, 22) << endl;  // true
    cout << "Example 1 (target 20): " << sol.hasPathSum(root, 20) << endl;  // false

    return 0;
}
