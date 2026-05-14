#include <iostream>
#include <vector>
using namespace std;

struct ListNode {
    int val;
    ListNode* next;
    ListNode(int x) : val(x), next(nullptr) {}
};

// Global pointer to track the successor node
ListNode* successor = nullptr;

/**
 * Reverse the first n nodes of a linked list.
 *
 * The global 'successor' variable tracks the node after the nth node,
 * which becomes the tail of the reversed sublist.
 */
ListNode* reverseN(ListNode* head, int n) {
    if (n == 1) {
        // Base case: mark successor as the node after the first node
        successor = head->next;
        return head;
    }

    // Recursively reverse the rest
    ListNode* new_head = reverseN(head->next, n - 1);

    // At this point, head->next points to the (n-1)th node
    // We want to reverse: head->next->next = head
    head->next->next = head;

    // Connect head to successor (the node after position n)
    head->next = successor;

    return new_head;
}

/**
 * Reverse a portion of a linked list from position left to right using recursion.
 *
 * Time Complexity: O(n) - traverse to left, then reverse to right
 * Space Complexity: O(n) - recursion call stack
 *
 * Strategy:
 * 1. If left == 1, call reverseN() to reverse the first 'right' nodes
 * 2. Otherwise, recursively process the next node with adjusted positions
 */
ListNode* reverseBetween(ListNode* head, int left, int right) {
    if (left == 1) {
        return reverseN(head, right);
    } else {
        // Recursively process the rest of the list
        head->next = reverseBetween(head->next, left - 1, right - 1);
        return head;
    }
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
