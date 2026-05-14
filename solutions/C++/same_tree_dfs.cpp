#include <iostream>
using namespace std;

struct TreeNode {
    int val;
    TreeNode* left;
    TreeNode* right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode* left, TreeNode* right)
        : val(x), left(left), right(right) {}
};

bool isSameTree(TreeNode* p, TreeNode* q) {
    /*
    Check if two binary trees are the same using DFS (recursive).

    Time Complexity: O(min(m, n)) where m and n are the number of nodes
    Space Complexity: O(min(h1, h2)) where h1 and h2 are the heights (call stack)
    */

    // Both nodes are nullptr (base case: equal)
    if (p == nullptr && q == nullptr) {
        return true;
    }

    // One is nullptr, the other isn't (not equal)
    if (p == nullptr || q == nullptr) {
        return false;
    }

    // Values differ (not equal)
    if (p->val != q->val) {
        return false;
    }

    // Recursively check left and right subtrees
    return isSameTree(p->left, q->left) && isSameTree(p->right, q->right);
}

int main() {
    // Example 1: Same trees
    //     1           1
    //    / \         / \
    //   2   3       2   3
    TreeNode* p1 = new TreeNode(1);
    p1->left = new TreeNode(2);
    p1->right = new TreeNode(3);

    TreeNode* q1 = new TreeNode(1);
    q1->left = new TreeNode(2);
    q1->right = new TreeNode(3);

    cout << (isSameTree(p1, q1) ? "true" : "false") << endl;  // Expected: true

    // Example 2: Different structure
    //     1           1
    //    /             \
    //   2               2
    TreeNode* p2 = new TreeNode(1);
    p2->left = new TreeNode(2);

    TreeNode* q2 = new TreeNode(1);
    q2->right = new TreeNode(2);

    cout << (isSameTree(p2, q2) ? "true" : "false") << endl;  // Expected: false

    // Example 3: Different values
    //     1           1
    //    / \         / \
    //   2   1       1   2
    TreeNode* p3 = new TreeNode(1);
    p3->left = new TreeNode(2);
    p3->right = new TreeNode(1);

    TreeNode* q3 = new TreeNode(1);
    q3->left = new TreeNode(1);
    q3->right = new TreeNode(2);

    cout << (isSameTree(p3, q3) ? "true" : "false") << endl;  // Expected: false

    // Example 4: Both empty
    cout << (isSameTree(nullptr, nullptr) ? "true" : "false") << endl;  // Expected: true

    return 0;
}
