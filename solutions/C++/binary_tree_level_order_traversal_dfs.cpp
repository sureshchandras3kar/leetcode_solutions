#include <iostream>
#include <vector>
using namespace std;

struct TreeNode { int val; TreeNode *left, *right; TreeNode(int x) : val(x), left(nullptr), right(nullptr) {} };

void dfs(TreeNode* n, int level, vector<vector<int>>& result) {
    if (!n) return;
    if (level == result.size()) result.push_back({});
    result[level].push_back(n->val);
    dfs(n->left, level + 1, result);
    dfs(n->right, level + 1, result);
}

vector<vector<int>> levelOrder(TreeNode* root) {
    vector<vector<int>> result;
    dfs(root, 0, result);
    return result;
}
