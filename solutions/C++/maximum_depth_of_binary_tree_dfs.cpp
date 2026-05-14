#include <algorithm>
#include <iostream>
using namespace std;

struct TreeNode {
    int val;
    TreeNode* left;
    TreeNode* right;
    TreeNode(int x = 0, TreeNode* l = nullptr, TreeNode* r = nullptr)
        : val(x), left(l), right(r) {}
};

int maxDepth(TreeNode* root) {
    /*
     * Find the maximum depth of a binary tree using DFS (recursive).
     *
     * Time Complexity: O(n) where n is the number of nodes
     * Space Complexity: O(h) where h is the height (call stack depth)
     */
    if (root == nullptr) {
        return 0;
    }

    return 1 + max(maxDepth(root->left), maxDepth(root->right));
}

// Test cases
int main() {
    // Example 1: [3,9,20,null,null,15,7]
    //       3
    //      / \
    //     9  20
    //       /  \
    //      15   7
    TreeNode* root1 = new TreeNode(3);
    root1->left = new TreeNode(9);
    root1->right = new TreeNode(20);
    root1->right->left = new TreeNode(15);
    root1->right->right = new TreeNode(7);

    cout << maxDepth(root1) << endl;  // Expected: 3

    // Example 2: [1,null,2]
    //     1
    //      \
    //       2
    TreeNode* root2 = new TreeNode(1);
    root2->right = new TreeNode(2);

    cout << maxDepth(root2) << endl;  // Expected: 2

    // Example 3: Empty tree
    TreeNode* root3 = nullptr;
    cout << maxDepth(root3) << endl;  // Expected: 0

    return 0;
}
