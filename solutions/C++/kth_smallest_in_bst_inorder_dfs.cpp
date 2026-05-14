#include <iostream>
using namespace std;

struct TreeNode {
    int val;
    TreeNode* left;
    TreeNode* right;
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
};

class Solution {
private:
    int count = 0;
    int result = -1;

    void inorder(TreeNode* node, int k) {
        if (!node || result != -1) return;

        // Traverse left subtree
        inorder(node->left, k);

        // Process current node
        count++;
        if (count == k) {
            result = node->val;
            return;
        }

        // Traverse right subtree
        inorder(node->right, k);
    }

public:
    int kthSmallest(TreeNode* root, int k) {
        inorder(root, k);
        return result;
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
