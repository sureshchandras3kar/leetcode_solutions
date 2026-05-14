#include <iostream>
#include <vector>
using namespace std;

struct ListNode {
    int val;
    ListNode* next;
    ListNode(int x) : val(x), next(nullptr) {}
};

class Solution {
public:
    ListNode* deleteDuplicates(ListNode* head) {
        /**
         * Remove all nodes with duplicate values from sorted linked list in single pass.
         *
         * Approach: Use a dummy node and prev pointer. When we find duplicates, skip all
         * nodes with that value by advancing current and updating prev->next.
         * Time: O(n) — single pass through the list
         * Space: O(1) — only pointer variables
         */
        if (!head) return nullptr;

        // Create dummy node to handle edge case where head is duplicate
        ListNode* dummy = new ListNode(0);
        dummy->next = head;
        ListNode* prev = dummy;
        ListNode* current = head;

        while (current) {
            // Check if current node is the start of a duplicate group
            if (current->next && current->val == current->next->val) {
                // Skip all nodes with the same value
                int value = current->val;
                while (current && current->val == value) {
                    current = current->next;
                }
                // Link prev to the first non-duplicate node
                prev->next = current;
            } else {
                // Current node is unique, keep it
                prev = current;
                current = current->next;
            }
        }

        ListNode* result = dummy->next;
        delete dummy;
        return result;
    }
};

// Test cases
void printList(ListNode* head) {
    while (head) {
        cout << head->val << " -> ";
        head = head->next;
    }
    cout << "null\n";
}

ListNode* createList(vector<int> arr) {
    if (arr.empty()) return nullptr;
    ListNode* head = new ListNode(arr[0]);
    ListNode* current = head;
    for (size_t i = 1; i < arr.size(); i++) {
        current->next = new ListNode(arr[i]);
        current = current->next;
    }
    return head;
}

int main() {
    Solution sol;

    // Test case 1: [1,2,3,3,4,4,5]
    ListNode* head1 = createList({1, 2, 3, 3, 4, 4, 5});
    cout << "Test 1: ";
    printList(sol.deleteDuplicates(head1));  // 1 -> 2 -> 5 -> null

    // Test case 2: [1,1,1,2,3]
    ListNode* head2 = createList({1, 1, 1, 2, 3});
    cout << "Test 2: ";
    printList(sol.deleteDuplicates(head2));  // 2 -> 3 -> null

    // Test case 3: [1,1]
    ListNode* head3 = createList({1, 1});
    cout << "Test 3: ";
    printList(sol.deleteDuplicates(head3));  // null

    return 0;
}
