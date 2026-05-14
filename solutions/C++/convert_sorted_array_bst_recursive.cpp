#include <iostream>
#include <vector>

struct TreeNode {
    int val;
    TreeNode* left;
    TreeNode* right;
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
};

TreeNode* sortedArrayToBST(std::vector<int>& nums, int left, int right) {
    if (left > right) return nullptr;

    int mid = left + (right - left) / 2;
    TreeNode* root = new TreeNode(nums[mid]);
    root->left = sortedArrayToBST(nums, left, mid - 1);
    root->right = sortedArrayToBST(nums, mid + 1, right);
    return root;
}

TreeNode* sortedArrayToBST(std::vector<int>& nums) {
    return sortedArrayToBST(nums, 0, nums.size() - 1);
}

int main() {
    std::vector<int> nums = {-10, -3, 0, 5, 9};
    TreeNode* root = sortedArrayToBST(nums);
    std::cout << root->val << std::endl;  // 0
    std::cout << root->left->val << std::endl;  // -3
    std::cout << root->right->val << std::endl;  // 5
    return 0;
}
