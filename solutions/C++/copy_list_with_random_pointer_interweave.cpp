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

        // First pass: create copies and interleave them
        // original -> copy -> original -> copy -> ...
        Node* current = head;
        while (current) {
            Node* copy = new Node(current->val);
            copy->next = current->next;
            current->next = copy;
            current = copy->next;
        }

        // Second pass: set random pointers for copy nodes
        current = head;
        while (current) {
            if (current->random) {
                current->next->random = current->random->next;
            }
            current = current->next->next;
        }

        // Third pass: restore original list and extract copy list
        current = head;
        Node* copyHead = head->next;
        while (current) {
            Node* copy = current->next;
            current->next = copy->next;
            copy->next = copy->next ? copy->next->next : NULL;
            current = current->next;
        }

        return copyHead;
    }
};
