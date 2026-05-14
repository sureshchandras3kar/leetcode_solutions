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
     * BFS Iterative approach to invert a binary tree.
     * Uses a queue to visit nodes level by level, swapping children.
     *
     * Time Complexity: O(n) - visit each node once
     * Space Complexity: O(w) - w = max width of tree (nodes at widest level)
     */
    TreeNode* invertTree(TreeNode* root) {
        if (root == nullptr) {
            return root;
        }

        queue<TreeNode*> q;
        q.push(root);

        while (!q.empty()) {
            TreeNode* node = q.front();
            q.pop();

            // Swap left and right children
            swap(node->left, node->right);

            // Add children to queue for processing
            if (node->left != nullptr) {
                q.push(node->left);
            }
            if (node->right != nullptr) {
                q.push(node->right);
            }
        }

        return root;
    }
};

int main() {
    // Create tree:     1
    //                /   \
    //               2     3
    TreeNode* root = new TreeNode(1);
    root->left = new TreeNode(2);
    root->right = new TreeNode(3);

    Solution sol;
    sol.invertTree(root);

    // Expected:        1
    //                /   \
    //               3     2
    cout << "Root: " << root->val << endl;  // 1
    cout << "Left: " << root->left->val << ", Right: " << root->right->val << endl;  // 3, 2

    return 0;
}
