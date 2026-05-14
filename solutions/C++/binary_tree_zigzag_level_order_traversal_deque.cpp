#include <iostream>
#include <vector>
#include <deque>
#include <queue>
using namespace std;

struct TreeNode { int val; TreeNode *left, *right; TreeNode(int x) : val(x), left(nullptr), right(nullptr) {} };

vector<vector<int>> zigzagLevelOrder(TreeNode* root) {
    vector<vector<int>> result;
    if (!root) return result;
    queue<TreeNode*> q; q.push(root);
    int level = 0;
    while (!q.empty()) {
        int sz = q.size(); deque<int> dq;
        for (int i = 0; i < sz; i++) {
            TreeNode* n = q.front(); q.pop();
            if (level % 2 == 0) dq.push_back(n->val);
            else dq.push_front(n->val);
            if (n->left) q.push(n->left);
            if (n->right) q.push(n->right);
        }
        result.push_back(vector<int>(dq.begin(), dq.end()));
        level++;
    }
    return result;
}
