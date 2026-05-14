#include <unordered_map>
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
         * Remove all nodes with duplicate values from sorted linked list using hash set.
         *
         * Approach: Count frequency of each value, then rebuild list with unique values only.
         * Time: O(n) — two passes through the list
         * Space: O(n) — hash map stores frequencies
         */
        if (!head) return nullptr;

        // Count frequencies
        unordered_map<int, int> freq;
        ListNode* current = head;
        while (current) {
            freq[current->val]++;
            current = current->next;
        }

        // Build result list with unique values only
        ListNode* dummy = new ListNode(0);
        dummy->next = head;
        ListNode* prev = dummy;
        current = head;

        while (current) {
            if (freq[current->val] == 1) {
                // Keep unique node
                prev = current;
                current = current->next;
            } else {
                // Skip duplicate node
                prev->next = current->next;
                current = current->next;
            }
        }

        ListNode* result = dummy->next;
        delete dummy;
        return result;
    }
};

// Test cases
#include <iostream>
#include <vector>

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
