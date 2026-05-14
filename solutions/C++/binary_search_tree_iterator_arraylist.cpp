#include <iostream>
#include <vector>
using namespace std;

// Definition for a binary tree node.
struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
};

class BSTIterator {
private:
    vector<int> arr;
    int index = 0;

    void inorder(TreeNode* node) {
        /**Pre-compute in-order traversal into array.*/
        if (!node) return;

        inorder(node->left);
        arr.push_back(node->val);
        inorder(node->right);
    }

public:
    BSTIterator(TreeNode* root) {
        /**
         * Binary Search Tree Iterator using pre-computed vector.
         * Stores all in-order elements upfront.
         * Space: O(n), next() O(1), hasNext() O(1)
         */
        inorder(root);
    }

    int next() {
        /**
         * Return next smallest element.
         * Time: O(1)
         */
        return arr[index++];
    }

    bool hasNext() {
        /**
         * Check if there are more elements.
         * Time: O(1)
         */
        return index < arr.size();
    }
};

// Test cases
int main() {
    // Example: [7,3,15,null,null,9,20]
    TreeNode* root = new TreeNode(7);
    root->left = new TreeNode(3);
    root->right = new TreeNode(15);
    root->right->left = new TreeNode(9);
    root->right->right = new TreeNode(20);

    BSTIterator iterator(root);
    while (iterator.hasNext()) {
        cout << iterator.next() << " ";
    }
    cout << endl;  // 3 7 9 15 20

    return 0;
}
