#include <iostream>
#include <climits>
using namespace std;

struct TreeNode {
    int val;
    TreeNode* left;
    TreeNode* right;
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
};

class Solution {
private:
    int minDiff = INT_MAX;
    TreeNode* prev = nullptr;

    void inorder(TreeNode* node) {
        if (!node) return;

        // Traverse left subtree
        inorder(node->left);

        // Process current node
        if (prev != nullptr) {
            minDiff = min(minDiff, node->val - prev->val);
        }
        prev = node;

        // Traverse right subtree
        inorder(node->right);
    }

public:
    int getMinimumDifference(TreeNode* root) {
        inorder(root);
        return minDiff;
    }
};

int main() {
    // Example 1
    TreeNode* root1 = new TreeNode(4);
    root1->left = new TreeNode(2);
    root1->right = new TreeNode(6);
    root1->left->left = new TreeNode(1);
    root1->left->right = new TreeNode(3);

    Solution sol;
    cout << sol.getMinimumDifference(root1) << endl;  // 1

    return 0;
}
