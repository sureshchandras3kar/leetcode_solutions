#include <iostream>
#include <vector>
#include <queue>

struct TreeNode {
    int val;
    TreeNode* left;
    TreeNode* right;
    TreeNode(int x = 0) : val(x), left(nullptr), right(nullptr) {}
};

TreeNode* sortedArrayToBST(std::vector<int>& nums) {
    if (nums.empty()) return nullptr;

    TreeNode* root = new TreeNode();
    std::queue<std::tuple<TreeNode*, int, int>> q;
    q.push({root, 0, (int)nums.size() - 1});

    while (!q.empty()) {
        auto [node, left, right] = q.front();
        q.pop();

        int mid = left + (right - left) / 2;
        node->val = nums[mid];

        if (left <= mid - 1) {
            node->left = new TreeNode();
            q.push({node->left, left, mid - 1});
        }

        if (mid + 1 <= right) {
            node->right = new TreeNode();
            q.push({node->right, mid + 1, right});
        }
    }

    return root;
}

int main() {
    std::vector<int> nums = {-10, -3, 0, 5, 9};
    TreeNode* root = sortedArrayToBST(nums);
    std::cout << root->val << std::endl;  // 0
    std::cout << root->left->val << std::endl;  // -3
    std::cout << root->right->val << std::endl;  // 5
    return 0;
}
