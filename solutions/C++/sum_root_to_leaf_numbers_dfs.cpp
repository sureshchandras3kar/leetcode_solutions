#include <iostream>
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
    int sumNumbers(TreeNode* root) {
        /**
         * Sum all root-to-leaf numbers using DFS.
         * Build number by appending digits as we traverse down.
         * Time: O(n), Space: O(h) for recursion stack
         */
        return dfs(root, 0);
    }

private:
    int dfs(TreeNode* node, int current_sum) {
        if (!node) return 0;

        // Build number: multiply by 10 and add current digit
        current_sum = current_sum * 10 + node->val;

        // Leaf node: return the complete number
        if (!node->left && !node->right) {
            return current_sum;
        }

        // Recursively process children and sum
        return dfs(node->left, current_sum) + dfs(node->right, current_sum);
    }
};

// Test cases
int main() {
    // Example 1: [1,2,3]
    TreeNode* root = new TreeNode(1);
    root->left = new TreeNode(2);
    root->right = new TreeNode(3);

    Solution sol;
    cout << "Example 1 sum: " << sol.sumNumbers(root) << endl;  // 25 (12 + 13)

    // Example 2: [4,9,0,5,1]
    TreeNode* root2 = new TreeNode(4);
    root2->left = new TreeNode(9);
    root2->right = new TreeNode(0);
    root2->left->left = new TreeNode(5);
    root2->left->right = new TreeNode(1);

    cout << "Example 2 sum: " << sol.sumNumbers(root2) << endl;  // 1026 (495 + 491 + 40)

    return 0;
}
