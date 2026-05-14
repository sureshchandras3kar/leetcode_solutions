#include <iostream>
#include <stack>
#include <climits>
using namespace std;

struct TreeNode {
    int val;
    TreeNode* left;
    TreeNode* right;
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
};

class Solution {
public:
    int getMinimumDifference(TreeNode* root) {
        stack<TreeNode*> st;
        TreeNode* current = root;
        TreeNode* prev = nullptr;
        int minDiff = INT_MAX;

        while (!st.empty() || current) {
            // Go to leftmost node
            while (current) {
                st.push(current);
                current = current->left;
            }

            // Current is null, pop from stack
            current = st.top();
            st.pop();

            // Process current node
            if (prev != nullptr) {
                minDiff = min(minDiff, current->val - prev->val);
            }
            prev = current;

            // Visit right subtree
            current = current->right;
        }

        return minDiff;
    }
};

int main() {
    TreeNode* root1 = new TreeNode(4);
    root1->left = new TreeNode(2);
    root1->right = new TreeNode(6);
    root1->left->left = new TreeNode(1);
    root1->left->right = new TreeNode(3);

    Solution sol;
    cout << sol.getMinimumDifference(root1) << endl;  // 1

    return 0;
}
