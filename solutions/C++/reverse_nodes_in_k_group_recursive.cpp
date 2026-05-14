#include <iostream>
#include <vector>
using namespace std;

struct ListNode {
    int val;
    ListNode* next;
    ListNode(int x) : val(x), next(nullptr) {}
};

ListNode* reverseKGroupRecursive(ListNode* head, int k) {
    /**
     * Reverse nodes in k-group recursively.
     *
     * Approach: Use recursion to handle each group independently.
     * - Check if there are k nodes remaining
     * - Reverse the first k nodes
     * - Recursively process the rest of the list
     * - Connect the reversed group to the result of the recursive call
     *
     * Time: O(n) - visit each node once
     * Space: O(n/k) - recursion depth is proportional to number of groups
     */

    // Find the k-th node
    ListNode* kth = head;
    for (int i = 0; i < k - 1 && kth; i++) {
        kth = kth->next;
    }

    // If no k-th node exists, cannot reverse, return original
    if (!kth) {
        return head;
    }

    // Save the next group's head (after k-th node)
    ListNode* next_group_head = kth->next;

    // Reverse from head to kth
    ListNode* prev = next_group_head;
    ListNode* curr = head;

    while (curr != next_group_head) {
        ListNode* next_temp = curr->next;
        curr->next = prev;
        prev = curr;
        curr = next_temp;
    }

    // head is now the tail of the reversed group
    // kth (now stored in prev) is the new head
    // Recursively process the remaining list and connect
    head->next = reverseKGroupRecursive(next_group_head, k);

    return prev;  // Return the new head (which was kth)
}

// Helper functions for testing
ListNode* createList(const vector<int>& values) {
    if (values.empty()) return nullptr;
    ListNode* head = new ListNode(values[0]);
    ListNode* current = head;
    for (int i = 1; i < values.size(); i++) {
        current->next = new ListNode(values[i]);
        current = current->next;
    }
    return head;
}

void printList(ListNode* head) {
    cout << "[";
    while (head) {
        cout << head->val;
        if (head->next) cout << ", ";
        head = head->next;
    }
    cout << "]" << endl;
}

// Test
int main() {
    ListNode* head1 = createList({1, 2, 3, 4, 5});
    printList(reverseKGroupRecursive(head1, 2));  // [2, 1, 4, 3, 5]

    ListNode* head2 = createList({1, 2, 3, 4, 5});
    printList(reverseKGroupRecursive(head2, 3));  // [3, 2, 1, 4, 5]

    ListNode* head3 = createList({1});
    printList(reverseKGroupRecursive(head3, 1));  // [1]

    return 0;
}
