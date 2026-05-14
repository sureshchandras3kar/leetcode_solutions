#include <iostream>
using namespace std;

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
                // Left is null, process current node
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
                    // Create link to current node
                    predecessor->right = current;
                    current = current->left;
                } else {
                    // Link exists, remove it and process current
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

int main() {
    TreeNode* root = new TreeNode(3);
    root->left = new TreeNode(1);
    root->right = new TreeNode(4);
    root->left->right = new TreeNode(2);

    Solution sol;
    cout << sol.kthSmallest(root, 1) << endl;  // 1
    cout << sol.kthSmallest(root, 3) << endl;  // 2

    return 0;
}
