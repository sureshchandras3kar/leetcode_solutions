#include <vector>
#include <unordered_map>

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
    unordered_map<int, int> inorder_map;

    TreeNode* build(vector<int>& preorder, int preorder_start, int preorder_end,
                    int inorder_start, int inorder_end) {
        if (preorder_start > preorder_end || inorder_start > inorder_end) {
            return nullptr;
        }

        // Root is the first element in preorder
        int root_val = preorder[preorder_start];
        TreeNode* root = new TreeNode(root_val);

        // Find root position in inorder
        int root_inorder_idx = inorder_map[root_val];

        // Number of elements in left subtree
        int left_size = root_inorder_idx - inorder_start;

        // Build left subtree
        root->left = build(preorder, preorder_start + 1, preorder_start + left_size,
                           inorder_start, root_inorder_idx - 1);

        // Build right subtree
        root->right = build(preorder, preorder_start + left_size + 1, preorder_end,
                            root_inorder_idx + 1, inorder_end);

        return root;
    }

public:
    TreeNode* buildTree(vector<int>& preorder, vector<int>& inorder) {
        if (preorder.empty() || inorder.empty()) {
            return nullptr;
        }

        // Create a map for quick lookup of indices in inorder
        for (int i = 0; i < inorder.size(); i++) {
            inorder_map[inorder[i]] = i;
        }

        return build(preorder, 0, preorder.size() - 1, 0, inorder.size() - 1);
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
