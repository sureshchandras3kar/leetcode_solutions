#include <iostream>
#include <climits>
#include <algorithm>
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
    long long max_sum = LLONG_MIN;

    long long dfs(TreeNode* node) {
        if (!node) return 0;

        // Max gain from left and right subtrees (at least 0 if negative)
        long long left_gain = max(0LL, dfs(node->left));
        long long right_gain = max(0LL, dfs(node->right));

        // Max path through this node (may bend at this node)
        long long max_path_through_node = node->val + left_gain + right_gain;
        max_sum = max(max_sum, max_path_through_node);

        // Return max path extending downward from this node
        return node->val + max(left_gain, right_gain);
    }

public:
    int maxPathSum(TreeNode* root) {
        /**
         * Find maximum path sum in binary tree using post-order DFS.
         * A path can pass through any node (not just root to leaf).
         * Time: O(n), Space: O(h) for recursion stack
         */
        dfs(root);
        return (int)max_sum;
    }
};

// Test cases
int main() {
    // Example 1: [1,2,3]
    TreeNode* root = new TreeNode(1);
    root->left = new TreeNode(2);
    root->right = new TreeNode(3);

    Solution sol;
    cout << "Example 1 max path: " << sol.maxPathSum(root) << endl;  // 6 (2 -> 1 -> 3)

    // Example 2: [-10,9,20,null,null,15,7]
    TreeNode* root2 = new TreeNode(-10);
    root2->left = new TreeNode(9);
    root2->right = new TreeNode(20);
    root2->right->left = new TreeNode(15);
    root2->right->right = new TreeNode(7);

    Solution sol2;
    cout << "Example 2 max path: " << sol2.maxPathSum(root2) << endl;  // 42 (15 -> 20 -> 7)

    return 0;
}
