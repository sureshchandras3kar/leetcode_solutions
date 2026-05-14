#include <unordered_map>
using namespace std;

class Node {
public:
    int val;
    Node* next;
    Node* random;

    Node(int _val) : val(_val), next(NULL), random(NULL) {}
};

class Solution {
public:
    Node* copyRandomList(Node* head) {
        if (!head) return NULL;

        // Map original nodes to their copies
        unordered_map<Node*, Node*> nodeMap;

        // First pass: create all copy nodes
        Node* current = head;
        while (current) {
            nodeMap[current] = new Node(current->val);
            current = current->next;
        }

        // Second pass: set next and random pointers
        current = head;
        while (current) {
            Node* copyNode = nodeMap[current];
            copyNode->next = current->next ? nodeMap[current->next] : NULL;
            copyNode->random = current->random ? nodeMap[current->random] : NULL;
            current = current->next;
        }

        return nodeMap[head];
    }
};
