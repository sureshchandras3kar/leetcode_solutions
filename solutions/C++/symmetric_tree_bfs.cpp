#include <iostream>
#include <queue>

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
     * Check if a binary tree is symmetric using BFS with a queue.
     *
     * Time Complexity: O(n) - visit each node once
     * Space Complexity: O(w) - queue width, where w is max nodes at any level
     */
    bool isSymmetric(TreeNode* root) {
        if (!root) {
            return true;
        }

        queue<pair<TreeNode*, TreeNode*>> q;
        q.push({root->left, root->right});

        while (!q.empty()) {
            auto [left, right] = q.front();
            q.pop();

            // Both nodes are null - continue (symmetric so far)
            if (!left && !right) {
                continue;
            }

            // One node is null or values differ - not symmetric
            if (!left || !right) {
                return false;
            }
            if (left->val != right->val) {
                return false;
            }

            // Add pairs for next level: left's left with right's right
            // and left's right with right's left (mirror pattern)
            q.push({left->left, right->right});
            q.push({left->right, right->left});
        }

        return true;
    }
};

// Test cases
int main() {
    Solution sol;

    // Example 1: Symmetric tree
    //       1
    //      / \
    //     2   2
    //    / \ / \
    //   3  4 4  3
    TreeNode* root1 = new TreeNode(1);
    root1->left = new TreeNode(2);
    root1->right = new TreeNode(2);
    root1->left->left = new TreeNode(3);
    root1->left->right = new TreeNode(4);
    root1->right->left = new TreeNode(4);
    root1->right->right = new TreeNode(3);
    cout << (sol.isSymmetric(root1) ? "true" : "false") << endl; // true

    // Example 2: Not symmetric
    //       1
    //      / \
    //     2   2
    //      \   \
    //       3   3
    TreeNode* root2 = new TreeNode(1);
    root2->left = new TreeNode(2);
    root2->right = new TreeNode(2);
    root2->left->right = new TreeNode(3);
    root2->right->right = new TreeNode(3);
    cout << (sol.isSymmetric(root2) ? "true" : "false") << endl; // false

    // Example 3: Single node
    TreeNode* root3 = new TreeNode(1);
    cout << (sol.isSymmetric(root3) ? "true" : "false") << endl; // true

    return 0;
}
