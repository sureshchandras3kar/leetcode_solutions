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
    int sumNumbers(TreeNode* root) {
        /**
         * Sum all root-to-leaf numbers using iterative BFS.
         * Queue stores (node, current_number) pairs.
         * Time: O(n), Space: O(w) where w is max width
         */
        if (!root) return 0;

        queue<pair<TreeNode*, int>> q;
        q.push({root, root->val});
        int total = 0;

        while (!q.empty()) {
            auto [node, current_sum] = q.front();
            q.pop();

            // Leaf node: add to total
            if (!node->left && !node->right) {
                total += current_sum;
                continue;
            }

            if (node->left) {
                q.push({node->left, current_sum * 10 + node->left->val});
            }
            if (node->right) {
                q.push({node->right, current_sum * 10 + node->right->val});
            }
        }

        return total;
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
