#include <iostream>
#include <vector>
using namespace std;

struct TreeNode {
    int val;
    TreeNode* left;
    TreeNode* right;
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
};

class Solution {
private:
    void dfs(TreeNode* node, int level, vector<int>& result) {
        if (!node) return;

        if (level == result.size()) {
            result.push_back(node->val);
        }

        dfs(node->right, level + 1, result);
        dfs(node->left, level + 1, result);
    }

public:
    vector<int> rightSideView(TreeNode* root) {
        vector<int> result;
        dfs(root, 0, result);
        return result;
    }
};

int main() {
    Solution sol;

    TreeNode* root1 = new TreeNode(1);
    root1->left = new TreeNode(2);
    root1->right = new TreeNode(3);
    root1->left->right = new TreeNode(5);
    root1->right->right = new TreeNode(4);

    vector<int> result1 = sol.rightSideView(root1);
    cout << "Right side view: ";
    for (int val : result1) cout << val << " ";
    cout << endl;  // Expected: 1 3 4

    return 0;
}
