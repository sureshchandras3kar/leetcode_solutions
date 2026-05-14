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
    TreeNode* lowestCommonAncestor(TreeNode* root, TreeNode* p, TreeNode* q) {
        TreeNode* current = root;

        while (current) {
            if (current->val > p->val && current->val > q->val) {
                // Both p and q are in left subtree
                current = current->left;
            } else if (current->val < p->val && current->val < q->val) {
                // Both p and q are in right subtree
                current = current->right;
            } else {
                // p and q are on different sides or one of them is current
                return current;
            }
        }

        return root;
    }
};

int main() {
    TreeNode* root = new TreeNode(6);
    root->left = new TreeNode(2);
    root->right = new TreeNode(8);
    root->left->left = new TreeNode(0);
    root->left->right = new TreeNode(4);
    root->left->right->left = new TreeNode(3);
    root->left->right->right = new TreeNode(5);
    root->right->left = new TreeNode(7);
    root->right->right = new TreeNode(9);

    Solution sol;
    TreeNode* p = root->left;
    TreeNode* q = root->left->right;
    cout << sol.lowestCommonAncestor(root, p, q)->val << endl;  // 2

    return 0;
}
