#include <iostream>
#include <vector>

using namespace std;

struct ListNode {
    int val;
    ListNode* next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode* next) : val(x), next(next) {}
};

/**
 * Partition linked list into two groups: values < x and values >= x.
 * Rearranges nodes in a single pass without creating separate lists.
 *
 * Time Complexity: O(n) where n is the number of nodes
 * Space Complexity: O(1) excluding the output list
 */
ListNode* partitionListSingleRearrange(ListNode* head, int x) {
    // Create a dummy node to simplify edge cases
    ListNode* dummy = new ListNode(0);
    dummy->next = head;

    // Find the node just before the partition point
    ListNode* prev = dummy;
    ListNode* current = head;

    while (current && current->val < x) {
        prev = current;
        current = current->next;
    }

    // Now prev is the last node with value < x (or dummy if all nodes >= x)
    // current is the first node with value >= x (or nullptr if all nodes < x)

    // If all nodes are less than x or list is empty, return as is
    if (!current) {
        return dummy->next;
    }

    // Now we need to insert nodes with value < x before the partition point
    ListNode* partition_prev = prev;  // Last node of < x group
    ListNode* run = current;  // Start from the partition point

    while (run) {
        if (run->val < x) {
            // Remove run from its position
            current->next = run->next;

            // Insert run before the partition point
            run->next = partition_prev->next;
            partition_prev->next = run;

            // Move partition_prev forward
            partition_prev = run;

            // Current stays the same to check the next node
            run = current->next;
        } else {
            // Move forward
            current = run;
            run = run->next;
        }
    }

    return dummy->next;
}

// Helper function to create a linked list from a vector
ListNode* createLinkedList(const vector<int>& values) {
    if (values.empty()) {
        return nullptr;
    }
    ListNode* head = new ListNode(values[0]);
    ListNode* current = head;
    for (size_t i = 1; i < values.size(); i++) {
        current->next = new ListNode(values[i]);
        current = current->next;
    }
    return head;
}

// Helper function to print linked list
void printLinkedList(ListNode* node) {
    cout << "[";
    while (node) {
        cout << node->val;
        if (node->next) cout << ", ";
        node = node->next;
    }
    cout << "]" << endl;
}

// Test cases
int main() {
    ListNode* head = createLinkedList({1, 4, 3, 2, 5, 2});
    ListNode* result = partitionListSingleRearrange(head, 3);
    printLinkedList(result);  // [1, 2, 2, 4, 3, 5]

    head = createLinkedList({2, 1});
    result = partitionListSingleRearrange(head, 2);
    printLinkedList(result);  // [1, 2]

    head = createLinkedList({5, 3, 1, 4, 2});
    result = partitionListSingleRearrange(head, 3);
    printLinkedList(result);  // [1, 2, 5, 3, 4]

    return 0;
}
