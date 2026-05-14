#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

struct TreeNode {
    int val;
    TreeNode* left;
    TreeNode* right;
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
};

class Solution {
public:
    int post_idx;

    TreeNode* buildTree(vector<int>& inorder, vector<int>& postorder) {
        /**
         * Construct binary tree from inorder and postorder traversal using index tracking.
         *
         * Key insight:
         * - Postorder: left subtree, right subtree, root (last element is always root)
         * - Inorder: left subtree, root, right subtree
         * - Use a pointer to track the current root in postorder (traverse from end to start)
         * - Find root in inorder to split into left and right subtrees
         *
         * Time Complexity: O(n²) in worst case due to linear search for root in inorder
         * Space Complexity: O(h) for recursion call stack where h is height
         */

        if (inorder.empty() || postorder.empty()) {
            return nullptr;
        }

        post_idx = postorder.size() - 1;
        return build(postorder, inorder, 0, inorder.size() - 1);
    }

private:
    TreeNode* build(vector<int>& postorder, vector<int>& inorder,
                   int in_start, int in_end) {
        /**
         * Recursively build tree by processing postorder from right to left.
         *
         * Args:
         *     in_start, in_end: Range in inorder array
         */

        if (in_start > in_end || post_idx < 0) {
            return nullptr;
        }

        // Current postorder element (processing from end to start)
        int root_val = postorder[post_idx];
        post_idx--;

        TreeNode* root = new TreeNode(root_val);

        // Find root position in inorder
        int root_idx = find(inorder.begin() + in_start, inorder.begin() + in_end + 1, root_val)
                       - inorder.begin();

        // Build right subtree first (postorder: left, right, root)
        // Since we traverse postorder backwards, right comes before left
        root->right = build(postorder, inorder, root_idx + 1, in_end);

        // Build left subtree
        root->left = build(postorder, inorder, in_start, root_idx - 1);

        return root;
    }
};

// Helper function for inorder traversal
void inorder_traversal(TreeNode* node, vector<int>& result) {
    if (!node) return;
    inorder_traversal(node->left, result);
    result.push_back(node->val);
    inorder_traversal(node->right, result);
}

// Helper function for postorder traversal
void postorder_traversal(TreeNode* node, vector<int>& result) {
    if (!node) return;
    postorder_traversal(node->left, result);
    postorder_traversal(node->right, result);
    result.push_back(node->val);
}

int main() {
    Solution sol;

    // Example 1: [3,9,20,null,null,15,7]
    //     3
    //    / \
    //   9  20
    //      / \
    //     15  7
    vector<int> inorder1 = {9, 3, 15, 20, 7};
    vector<int> postorder1 = {9, 15, 7, 20, 3};
    TreeNode* root1 = sol.buildTree(inorder1, postorder1);

    vector<int> result1;
    inorder_traversal(root1, result1);
    cout << "Inorder: ";
    for (int val : result1) cout << val << " ";
    cout << "\n";  // Expected: 9 3 15 20 7

    vector<int> result1_post;
    postorder_traversal(root1, result1_post);
    cout << "Postorder: ";
    for (int val : result1_post) cout << val << " ";
    cout << "\n";  // Expected: 9 15 7 20 3

    return 0;
}
