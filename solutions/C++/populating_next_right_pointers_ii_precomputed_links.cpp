#include <iostream>
using namespace std;

// Definition for a Node.
class Node {
public:
    int val;
    Node* left;
    Node* right;
    Node* next;

    Node() : val(0), left(NULL), right(NULL), next(NULL) {}
    Node(int _val) : val(_val), left(NULL), right(NULL), next(NULL) {}
};

class Solution {
public:
    Node* connect(Node* root) {
        /**
         * Populates next pointers using pre-computed links without queue.
         * Uses next pointers of parent level to traverse current level.
         * Time: O(n), Space: O(1)
         */
        if (!root) return root;

        Node* leftmost = root;

        while (leftmost) {
            Node* prev = NULL;
            Node* current = leftmost;

            while (current) {
                if (current->left) {
                    if (prev) {
                        prev->next = current->left;
                    }
                    prev = current->left;
                }

                if (current->right) {
                    if (prev) {
                        prev->next = current->right;
                    }
                    prev = current->right;
                }

                current = current->next;
            }

            leftmost = leftmost->left;
        }

        return root;
    }
};

// Test cases
int main() {
    // Example 1: Tree with next pointers connected
    Node* root = new Node(1);
    root->left = new Node(2);
    root->right = new Node(3);
    root->left->left = new Node(4);
    root->left->right = new Node(5);
    root->right->right = new Node(7);

    Solution sol;
    sol.connect(root);
    cout << "Example 1: Tree with next pointers connected via pre-computed links" << endl;

    return 0;
}
