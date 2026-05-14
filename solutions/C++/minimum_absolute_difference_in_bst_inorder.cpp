#include <climits>
#include <algorithm>

struct TreeNode {
    int val;
    TreeNode* left;
    TreeNode* right;
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
};

class Solution {
private:
    int min_diff = INT_MAX;
    int* prev = nullptr;

    void inorder(TreeNode* node) {
        if (!node) return;

        inorder(node->left);

        if (prev != nullptr) {
            min_diff = std::min(min_diff, node->val - *prev);
        }
        prev = new int(node->val);

        inorder(node->right);
    }

public:
    int getMinimumDifference(TreeNode* root) {
        inorder(root);
        return min_diff;
    }
};

// Test
int main() {
    TreeNode* root = new TreeNode(4);
    root->left = new TreeNode(2);
    root->right = new TreeNode(6);
    root->left->left = new TreeNode(1);
    root->left->right = new TreeNode(3);

    Solution sol;
    std::cout << sol.getMinimumDifference(root) << std::endl;  // 1
    return 0;
}
