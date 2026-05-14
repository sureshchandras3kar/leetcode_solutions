#include <iostream>
#include <stack>
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
    stack<TreeNode*> st;

    void pushLeft(TreeNode* node) {
        /**Push all left nodes onto stack.*/
        while (node) {
            st.push(node);
            node = node->left;
        }
    }

public:
    BSTIterator(TreeNode* root) {
        /**
         * Binary Search Tree Iterator using stack for in-order traversal.
         * Implements lazy evaluation: next() O(1) amortized, hasNext() O(1).
         * Space: O(h) where h is height
         */
        pushLeft(root);
    }

    int next() {
        /**
         * Return next smallest element.
         * Time: O(1) amortized
         */
        TreeNode* node = st.top();
        st.pop();

        if (node->right) {
            pushLeft(node->right);
        }

        return node->val;
    }

    bool hasNext() {
        /**
         * Check if there are more elements.
         * Time: O(1)
         */
        return !st.empty();
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
