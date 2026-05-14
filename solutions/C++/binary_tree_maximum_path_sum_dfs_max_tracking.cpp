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
public:
    int maxPathSum(TreeNode* root) {
        /**
         * Find maximum path sum using DFS with class-level max tracking.
         * Maintains max_sum as instance variable updated during traversal.
         * Time: O(n), Space: O(h) for recursion stack
         */
        long long max_sum = LLONG_MIN;
        dfs(root, max_sum);
        return (int)max_sum;
    }

private:
    long long dfs(TreeNode* node, long long& max_sum) {
        if (!node) return 0;

        // Max single-path sum extending from this node
        long long left_sum = max(0LL, dfs(node->left, max_sum));
        long long right_sum = max(0LL, dfs(node->right, max_sum));

        // Path bending at this node
        long long path_sum = node->val + left_sum + right_sum;
        max_sum = max(max_sum, path_sum);

        // Return best single path extending downward
        return node->val + max(left_sum, right_sum);
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
