#include <vector>
#include <algorithm>

using namespace std;

struct TreeNode {
    int val;
    TreeNode* left;
    TreeNode* right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode* left, TreeNode* right) : val(x), left(left), right(right) {}
};

class Solution {
private:
    int preorder_idx;

    TreeNode* build(vector<int>& preorder, vector<int>& inorder,
                    int inorder_start, int inorder_end) {
        if (inorder_start > inorder_end || preorder_idx >= preorder.size()) {
            return nullptr;
        }

        // Root is the current element in preorder
        int root_val = preorder[preorder_idx];
        preorder_idx++;
        TreeNode* root = new TreeNode(root_val);

        // Find root position in inorder
        auto it = find(inorder.begin() + inorder_start, inorder.begin() + inorder_end + 1, root_val);
        int root_inorder_idx = distance(inorder.begin(), it);

        // Build left subtree
        root->left = build(preorder, inorder, inorder_start, root_inorder_idx - 1);

        // Build right subtree
        root->right = build(preorder, inorder, root_inorder_idx + 1, inorder_end);

        return root;
    }

public:
    TreeNode* buildTree(vector<int>& preorder, vector<int>& inorder) {
        if (preorder.empty() || inorder.empty()) {
            return nullptr;
        }

        preorder_idx = 0;
        return build(preorder, inorder, 0, inorder.size() - 1);
    }
};

// Test cases
int main() {
    Solution sol;

    // Test 1: [3,9,20,15,7], [9,3,15,20,7]
    vector<int> preorder1 = {3, 9, 20, 15, 7};
    vector<int> inorder1 = {9, 3, 15, 20, 7};
    TreeNode* root1 = sol.buildTree(preorder1, inorder1);
    printf("Test 1 - Root: %d\n", root1->val);  // 3

    // Test 2: [1], [1]
    vector<int> preorder2 = {1};
    vector<int> inorder2 = {1};
    TreeNode* root2 = sol.buildTree(preorder2, inorder2);
    printf("Test 2 - Root: %d\n", root2->val);  // 1

    return 0;
}
