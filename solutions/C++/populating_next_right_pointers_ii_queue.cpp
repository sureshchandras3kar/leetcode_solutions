#include <queue>
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
         * Populates next pointers using level-order BFS queue.
         * Time: O(n), Space: O(w) where w is max width
         */
        if (!root) return root;

        queue<Node*> q;
        q.push(root);

        while (!q.empty()) {
            int level_size = q.size();
            Node* prev = NULL;

            for (int i = 0; i < level_size; i++) {
                Node* node = q.front();
                q.pop();

                if (prev) {
                    prev->next = node;
                }
                prev = node;

                if (node->left) q.push(node->left);
                if (node->right) q.push(node->right);
            }
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
    cout << "Example 1: Tree with next pointers connected via queue" << endl;

    return 0;
}
