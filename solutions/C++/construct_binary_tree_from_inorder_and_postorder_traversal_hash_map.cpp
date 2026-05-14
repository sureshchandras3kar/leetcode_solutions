#include <iostream>
#include <vector>
#include <unordered_map>
#include <string>

using namespace std;

struct TreeNode {
    int val;
    TreeNode* left;
    TreeNode* right;
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
};

class Solution {
public:
    unordered_map<int, int> inorder_map;

    TreeNode* buildTree(vector<int>& inorder, vector<int>& postorder) {
        /**
         * Construct binary tree from inorder and postorder traversal using HashMap.
         *
         * Key insight:
         * - Postorder: left subtree, right subtree, root (last element is always root)
         * - Inorder: left subtree, root, right subtree
         *
         * Use a HashMap to quickly find the root's position in inorder,
         * then recursively build left and right subtrees.
         *
         * Time Complexity: O(n) where n is the number of nodes
         * Space Complexity: O(n) for HashMap and recursion call stack
         */

        if (inorder.empty() || postorder.empty()) {
            return nullptr;
        }

        // Build HashMap for O(1) inorder lookup
        for (int i = 0; i < inorder.size(); ++i) {
            inorder_map[inorder[i]] = i;
        }

        return build(postorder, 0, postorder.size() - 1, 0, inorder.size() - 1);
    }

private:
    TreeNode* build(vector<int>& postorder, int post_start, int post_end,
                    int in_start, int in_end) {
        /**
         * Recursively build tree from postorder and inorder ranges.
         *
         * Args:
         *     post_start, post_end: Range in postorder array
         *     in_start, in_end: Range in inorder array
         */

        if (post_start > post_end || in_start > in_end) {
            return nullptr;
        }

        // Last element in postorder range is the root
        int root_val = postorder[post_end];
        TreeNode* root = new TreeNode(root_val);

        // Find root position in inorder
        int root_idx = inorder_map[root_val];

        // Number of nodes in left subtree
        int left_size = root_idx - in_start;

        // Recursively build left subtree
        root->left = build(postorder, post_start, post_start + left_size - 1,
                          in_start, root_idx - 1);

        // Recursively build right subtree
        root->right = build(postorder, post_start + left_size, post_end - 1,
                           root_idx + 1, in_end);

        return root;
    }
};

// Helper function for inorder traversal
vector<int> inorder_traversal(TreeNode* node) {
    if (!node) return {};
    vector<int> result;
    inorder_traversal(node->left, result);
    result.push_back(node->val);
    inorder_traversal(node->right, result);
    return result;
}

void inorder_traversal(TreeNode* node, vector<int>& result) {
    if (!node) return;
    inorder_traversal(node->left, result);
    result.push_back(node->val);
    inorder_traversal(node->right, result);
}

// Helper function for postorder traversal
vector<int> postorder_traversal(TreeNode* node) {
    if (!node) return {};
    vector<int> result;
    postorder_traversal(node->left, result);
    postorder_traversal(node->right, result);
    result.push_back(node->val);
    return result;
}

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

    vector<int> result1 = inorder_traversal(root1);
    cout << "Inorder: ";
    for (int val : result1) cout << val << " ";
    cout << "\n";  // Expected: 9 3 15 20 7

    vector<int> result1_post = postorder_traversal(root1);
    cout << "Postorder: ";
    for (int val : result1_post) cout << val << " ";
    cout << "\n";  // Expected: 9 15 7 20 3

    return 0;
}
