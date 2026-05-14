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
 * Uses two separate list heads and merges them at the end.
 *
 * Time Complexity: O(n) where n is the number of nodes
 * Space Complexity: O(1) excluding the output list
 */
ListNode* partitionListTwoHeads(ListNode* head, int x) {
    // Create dummy nodes for two lists
    ListNode* smaller_dummy = new ListNode(0);
    ListNode* larger_dummy = new ListNode(0);

    // Pointers to build the two lists
    ListNode* smaller_ptr = smaller_dummy;
    ListNode* larger_ptr = larger_dummy;

    // Partition nodes into two lists
    ListNode* current = head;
    while (current) {
        if (current->val < x) {
            smaller_ptr->next = current;
            smaller_ptr = smaller_ptr->next;
        } else {
            larger_ptr->next = current;
            larger_ptr = larger_ptr->next;
        }
        current = current->next;
    }

    // Close the larger list to avoid cycles
    larger_ptr->next = nullptr;

    // Connect the two lists
    smaller_ptr->next = larger_dummy->next;

    return smaller_dummy->next;
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
    ListNode* result = partitionListTwoHeads(head, 3);
    printLinkedList(result);  // [1, 2, 2, 4, 3, 5]

    head = createLinkedList({2, 1});
    result = partitionListTwoHeads(head, 2);
    printLinkedList(result);  // [1, 2]

    head = createLinkedList({5, 3, 1, 4, 2});
    result = partitionListTwoHeads(head, 3);
    printLinkedList(result);  // [1, 2, 5, 3, 4]

    return 0;
}
