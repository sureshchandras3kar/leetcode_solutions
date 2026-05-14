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
    void dfs(TreeNode* node, int level, vector<long long>& sums, vector<int>& counts) {
        if (!node) return;

        if (level == sums.size()) {
            sums.push_back(0);
            counts.push_back(0);
        }

        sums[level] += node->val;
        counts[level]++;

        dfs(node->left, level + 1, sums, counts);
        dfs(node->right, level + 1, sums, counts);
    }

public:
    vector<double> averageOfLevels(TreeNode* root) {
        vector<long long> sums;
        vector<int> counts;
        dfs(root, 0, sums, counts);

        vector<double> result;
        for (int i = 0; i < sums.size(); i++) {
            result.push_back((double)sums[i] / counts[i]);
        }
        return result;
    }
};

int main() {
    Solution sol;
    TreeNode* root = new TreeNode(3);
    root->left = new TreeNode(9);
    root->right = new TreeNode(20);
    root->right->left = new TreeNode(15);
    root->right->right = new TreeNode(7);

    vector<double> result = sol.averageOfLevels(root);
    cout << "Averages: ";
    for (double val : result) cout << val << " ";
    cout << endl;

    return 0;
}
