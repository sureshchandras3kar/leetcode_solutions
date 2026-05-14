#include <iostream>
#include <vector>
using namespace std;

struct ListNode {
    int val;
    ListNode* next;
    ListNode(int x) : val(x), next(nullptr) {}
};

/**
 * Reverse a portion of a linked list from position left to right (1-indexed).
 *
 * Time Complexity: O(n) - single pass through the list
 * Space Complexity: O(1) - only pointer variables
 *
 * Strategy:
 * 1. Create a dummy node to handle edge case of reversing from head
 * 2. Advance prev pointer to position left-1
 * 3. Perform (right - left) reversals by moving nodes to the front of the sublist
 * 4. Return dummy->next
 */
ListNode* reverseBetween(ListNode* head, int left, int right) {
    if (left == right) {
        return head;
    }

    // Create dummy node pointing to head
    ListNode* dummy = new ListNode(0);
    dummy->next = head;
    ListNode* prev = dummy;

    // Advance prev to position left-1
    for (int i = 0; i < left - 1; i++) {
        prev = prev->next;
    }

    // curr is the first node to reverse (at position left)
    ListNode* curr = prev->next;

    // Perform (right - left) reversals
    for (int i = 0; i < right - left; i++) {
        // next_temp is the node we want to move to the front
        ListNode* next_temp = curr->next;
        // Bypass next_temp by connecting curr to next_temp->next
        curr->next = next_temp->next;
        // Move next_temp to the front of the sublist
        next_temp->next = prev->next;
        prev->next = next_temp;
    }

    ListNode* result = dummy->next;
    delete dummy;
    return result;
}

// Helper function to create a linked list from a vector
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

// Helper function to print a linked list
void printList(ListNode* head) {
    while (head) {
        cout << head->val << " ";
        head = head->next;
    }
    cout << "\n";
}

// Helper function to delete a linked list
void deleteList(ListNode* head) {
    while (head) {
        ListNode* temp = head;
        head = head->next;
        delete temp;
    }
}

// Test cases
int main() {
    // Test 1: Reverse middle portion
    ListNode* head1 = createList({1, 2, 3, 4, 5});
    ListNode* result1 = reverseBetween(head1, 2, 4);
    cout << "Test 1: ";
    printList(result1);  // 1 4 3 2 5
    deleteList(result1);

    // Test 2: Single node (no reversal)
    ListNode* head2 = createList({5});
    ListNode* result2 = reverseBetween(head2, 1, 1);
    cout << "Test 2: ";
    printList(result2);  // 5
    deleteList(result2);

    // Test 3: Reverse entire list
    ListNode* head3 = createList({1, 2});
    ListNode* result3 = reverseBetween(head3, 1, 2);
    cout << "Test 3: ";
    printList(result3);  // 2 1
    deleteList(result3);

    // Test 4: Reverse from start
    ListNode* head4 = createList({1, 2, 3, 4, 5});
    ListNode* result4 = reverseBetween(head4, 1, 3);
    cout << "Test 4: ";
    printList(result4);  // 3 2 1 4 5
    deleteList(result4);

    // Test 5: Reverse at end
    ListNode* head5 = createList({1, 2, 3, 4, 5});
    ListNode* result5 = reverseBetween(head5, 3, 5);
    cout << "Test 5: ";
    printList(result5);  // 1 2 5 4 3
    deleteList(result5);

    return 0;
}
