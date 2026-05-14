#include <iostream>
#include <climits>
using namespace std;

struct TreeNode { int val; TreeNode *left, *right; TreeNode(int x) : val(x), left(nullptr), right(nullptr) {} };

bool dfs(TreeNode* n, long long minVal, long long maxVal) {
    if (!n) return true;
    if (n->val <= minVal || n->val >= maxVal) return false;
    return dfs(n->left, minVal, n->val) && dfs(n->right, n->val, maxVal);
}

bool isValidBST(TreeNode* root) {
    return dfs(root, LLONG_MIN, LLONG_MAX);
}
