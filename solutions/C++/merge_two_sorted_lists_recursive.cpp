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
 * Merge two sorted linked lists recursively.
 *
 * Time Complexity: O(m + n) where m and n are the lengths of list1 and list2
 * Space Complexity: O(m + n) due to the recursion call stack
 */
ListNode* mergeTwoSortedListsRecursive(ListNode* list1, ListNode* list2) {
    // Base cases: if one list is empty, return the other
    if (!list1) {
        return list2;
    }
    if (!list2) {
        return list1;
    }

    // Compare the values and recursively merge
    if (list1->val <= list2->val) {
        list1->next = mergeTwoSortedListsRecursive(list1->next, list2);
        return list1;
    } else {
        list2->next = mergeTwoSortedListsRecursive(list1, list2->next);
        return list2;
    }
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
    ListNode* list1 = createLinkedList({1, 2, 4});
    ListNode* list2 = createLinkedList({1, 3, 4});
    ListNode* result = mergeTwoSortedListsRecursive(list1, list2);
    printLinkedList(result);  // [1, 1, 2, 3, 4, 4]

    list1 = createLinkedList({});
    list2 = createLinkedList({});
    result = mergeTwoSortedListsRecursive(list1, list2);
    printLinkedList(result);  // []

    list1 = createLinkedList({});
    list2 = createLinkedList({0});
    result = mergeTwoSortedListsRecursive(list1, list2);
    printLinkedList(result);  // [0]

    return 0;
}
