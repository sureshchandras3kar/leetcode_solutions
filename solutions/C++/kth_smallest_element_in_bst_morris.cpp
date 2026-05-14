#include <iostream>

struct TreeNode {
    int val;
    TreeNode* left;
    TreeNode* right;
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
};

class Solution {
public:
    int kthSmallest(TreeNode* root, int k) {
        int count = 0;
        TreeNode* current = root;

        while (current) {
            if (!current->left) {
                // No left child: visit current
                count++;
                if (count == k) {
                    return current->val;
                }
                current = current->right;
            } else {
                // Find in-order predecessor
                TreeNode* predecessor = current->left;
                while (predecessor->right && predecessor->right != current) {
                    predecessor = predecessor->right;
                }

                if (!predecessor->right) {
                    // First visit: create thread
                    predecessor->right = current;
                    current = current->left;
                } else {
                    // Second visit: remove thread, process current
                    predecessor->right = nullptr;
                    count++;
                    if (count == k) {
                        return current->val;
                    }
                    current = current->right;
                }
            }
        }

        return -1;
    }
};

// Test
int main() {
    TreeNode* root = new TreeNode(3);
    root->left = new TreeNode(1);
    root->right = new TreeNode(4);
    root->left->right = new TreeNode(2);

    Solution sol;
    std::cout << sol.kthSmallest(root, 1) << std::endl;  // 1
    std::cout << sol.kthSmallest(root, 3) << std::endl;  // 2
    return 0;
}
