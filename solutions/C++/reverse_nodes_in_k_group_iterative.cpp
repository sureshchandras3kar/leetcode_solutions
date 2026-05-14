#include <iostream>
#include <vector>
using namespace std;

struct ListNode {
    int val;
    ListNode* next;
    ListNode(int x) : val(x), next(nullptr) {}
};

ListNode* reverseKGroupIterative(ListNode* head, int k) {
    /**
     * Reverse nodes in k-group iteratively.
     *
     * Approach: Reverse each group of k nodes in-place using iteration.
     * - Find the k-th node to determine the group boundary
     * - Reverse the group from current to k-th node
     * - Link the reversed group back to the previous group
     * - Move to the next group and repeat
     *
     * Time: O(n) - visit each node once
     * Space: O(1) - only pointers, no extra structures
     */

    auto get_kth_node = [](ListNode* node, int k) -> ListNode* {
        while (node && k > 1) {
            node = node->next;
            k--;
        }
        return node;
    };

    // Dummy node to simplify logic
    ListNode* dummy = new ListNode(0);
    dummy->next = head;

    ListNode* prev_group = dummy;

    while (true) {
        // Check if there are at least k nodes remaining
        ListNode* kth_node = get_kth_node(prev_group, k + 1);
        if (!kth_node) {
            break;
        }

        // Mark the start and end of the current group
        ListNode* group_start = prev_group->next;
        ListNode* group_end = kth_node;

        // Save the next group's start
        ListNode* next_group_start = group_end->next;

        // Reverse the current group
        ListNode* prev = next_group_start;
        ListNode* curr = group_start;

        while (curr != next_group_start) {
            ListNode* next_temp = curr->next;
            curr->next = prev;
            prev = curr;
            curr = next_temp;
        }

        // Link the previous group to the reversed current group
        prev_group->next = group_end;
        prev_group = group_start;
    }

    ListNode* result = dummy->next;
    delete dummy;
    return result;
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
    printList(reverseKGroupIterative(head1, 2));  // [2, 1, 4, 3, 5]

    ListNode* head2 = createList({1, 2, 3, 4, 5});
    printList(reverseKGroupIterative(head2, 3));  // [3, 2, 1, 4, 5]

    ListNode* head3 = createList({1});
    printList(reverseKGroupIterative(head3, 1));  // [1]

    return 0;
}
