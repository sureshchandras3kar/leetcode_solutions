#include <iostream>
#include <queue>
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
         * Check if tree has root-to-leaf path summing to targetSum using iterative BFS.
         * Queue stores (node, current_sum) pairs.
         * Time: O(n), Space: O(w) where w is max width
         */
        if (!root) return false;

        queue<pair<TreeNode*, int>> q;
        q.push({root, root->val});

        while (!q.empty()) {
            auto [node, current_sum] = q.front();
            q.pop();

            // Check leaf node
            if (!node->left && !node->right && current_sum == targetSum) {
                return true;
            }

            if (node->left) {
                q.push({node->left, current_sum + node->left->val});
            }
            if (node->right) {
                q.push({node->right, current_sum + node->right->val});
            }
        }

        return false;
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
