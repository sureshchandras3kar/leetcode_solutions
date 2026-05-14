#include <iostream>
#include <climits>
using namespace std;

struct TreeNode { int val; TreeNode *left, *right; TreeNode(int x) : val(x), left(nullptr), right(nullptr) {} };

bool isValidBST(TreeNode* root) {
    long long prev = LLONG_MIN;
    function<bool(TreeNode*)> dfs = [&](TreeNode* n) -> bool {
        if (!n) return true;
        if (!dfs(n->left)) return false;
        if (n->val <= prev) return false;
        prev = n->val;
        return dfs(n->right);
    };
    return dfs(root);
}
