#include <iostream>
using namespace std;

// Definition for a binary tree node.
struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
};

class Solution {
private:
    bool exists(long long pos, int height, TreeNode* root) {
        /**Check if node at position pos exists (0-indexed, 1-based actual position).*/
        long long left = 0, right = (1LL << (height - 1)) - 1;

        for (int i = 0; i < height - 1; i++) {
            long long mid = (left + right + 1) / 2;
            if (pos >= mid) {
                root = root->right;
                left = mid;
            } else {
                root = root->left;
                right = mid - 1;
            }
        }

        return root != nullptr;
    }

public:
    int countNodes(TreeNode* root) {
        /**
         * Count nodes in complete binary tree using binary search on node positions.
         * Uses existence check for node at each possible position.
         * Time: O(log² n), Space: O(log n) for recursion
         */
        if (!root) return 0;

        // Find height of tree
        int height = 0;
        TreeNode* node = root;
        while (node) {
            height++;
            node = node->left;
        }

        // Binary search on number of nodes
        // For a complete tree of height h, nodes range from 2^(h-1) to 2^h - 1
        long long low = 1LL << (height - 1);   // 2^(h-1)
        long long high = (1LL << height) - 1;  // 2^h - 1

        while (low <= high) {
            long long mid = (low + high + 1) / 2;
            if (exists(mid, height, root)) {
                low = mid;
            } else {
                high = mid - 1;
            }
        }

        return (int)low;
    }
};

// Test cases
int main() {
    // Example 1: Complete tree with 5 nodes
    TreeNode* root = new TreeNode(1);
    root->left = new TreeNode(2);
    root->right = new TreeNode(3);
    root->left->left = new TreeNode(4);
    root->left->right = new TreeNode(5);

    Solution sol;
    cout << "Example 1 node count: " << sol.countNodes(root) << endl;  // 5

    // Example 2: Single node
    TreeNode* single = new TreeNode(1);
    cout << "Example 2 node count: " << sol.countNodes(single) << endl;  // 1

    // Example 3: Empty tree
    cout << "Example 3 node count: " << sol.countNodes(nullptr) << endl;  // 0

    return 0;
}
