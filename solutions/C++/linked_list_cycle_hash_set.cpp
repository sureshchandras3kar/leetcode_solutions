#include <iostream>
#include <unordered_set>
using namespace std;

struct ListNode {
    int val;
    ListNode* next;
    ListNode(int x) : val(x), next(nullptr) {}
};

class Solution {
public:
    bool hasCycle(ListNode* head) {
        unordered_set<ListNode*> seen;
        ListNode* current = head;

        while (current) {
            if (seen.count(current)) {
                return true;
            }
            seen.insert(current);
            current = current->next;
        }

        return false;
    }
};

int main() {
    Solution solution;

    // Test case 1: Cycle exists
    ListNode* node1 = new ListNode(3);
    ListNode* node2 = new ListNode(2);
    ListNode* node3 = new ListNode(0);
    ListNode* node4 = new ListNode(-4);
    node1->next = node2;
    node2->next = node3;
    node3->next = node4;
    node4->next = node2;  // Cycle
    cout << "Cycle exists: " << (solution.hasCycle(node1) ? "true" : "false") << endl;  // true

    // Test case 2: No cycle
    ListNode* node5 = new ListNode(1);
    ListNode* node6 = new ListNode(2);
    node5->next = node6;
    cout << "Cycle exists: " << (solution.hasCycle(node5) ? "true" : "false") << endl;  // false

    // Test case 3: Single node
    ListNode* node7 = new ListNode(1);
    cout << "Cycle exists: " << (solution.hasCycle(node7) ? "true" : "false") << endl;  // false

    return 0;
}
