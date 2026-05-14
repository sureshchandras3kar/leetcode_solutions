#include <iostream>
#include <vector>

struct ListNode {
    int val;
    ListNode* next;
    ListNode(int x = 0) : val(x), next(nullptr) {}
};

ListNode* helper(ListNode* l1, ListNode* l2, int carry) {
    // Base case: both lists are empty and no carry
    if (!l1 && !l2 && carry == 0) {
        return nullptr;
    }

    int val1 = l1 ? l1->val : 0;
    int val2 = l2 ? l2->val : 0;

    int total = val1 + val2 + carry;
    int digit = total % 10;
    int newCarry = total / 10;

    // Move to next nodes
    ListNode* nextL1 = l1 ? l1->next : nullptr;
    ListNode* nextL2 = l2 ? l2->next : nullptr;

    // Recursively build the rest of the list
    ListNode* nextNode = helper(nextL1, nextL2, newCarry);

    // Create node with current digit
    return new ListNode(digit, nextNode);
}

ListNode* addTwoNumbers(ListNode* l1, ListNode* l2) {
    /*
    Add two numbers represented by linked lists in reverse order using recursion.

    Time Complexity: O(max(m, n)) where m and n are the lengths of the lists
    Space Complexity: O(max(m, n)) for the recursion call stack and output list
    */
    return helper(l1, l2, 0);
}

// Helper function to create linked list from vector
ListNode* createLinkedList(const std::vector<int>& arr) {
    if (arr.empty()) return nullptr;
    ListNode* head = new ListNode(arr[0]);
    ListNode* current = head;
    for (size_t i = 1; i < arr.size(); i++) {
        current->next = new ListNode(arr[i]);
        current = current->next;
    }
    return head;
}

// Helper function to print linked list
void printLinkedList(ListNode* head) {
    std::cout << "[";
    bool first = true;
    while (head) {
        if (!first) std::cout << ", ";
        std::cout << head->val;
        head = head->next;
        first = false;
    }
    std::cout << "]" << std::endl;
}

// Helper function to delete linked list
void deleteLinkedList(ListNode* head) {
    while (head) {
        ListNode* temp = head;
        head = head->next;
        delete temp;
    }
}

int main() {
    // Test case 1: [2,4,3] + [5,6,4] = [7,0,8] (342 + 465 = 807)
    ListNode* l1 = createLinkedList({2, 4, 3});
    ListNode* l2 = createLinkedList({5, 6, 4});
    ListNode* result = addTwoNumbers(l1, l2);
    std::cout << "Test 1: ";
    printLinkedList(result);
    deleteLinkedList(result);

    // Test case 2: [0] + [0] = [0]
    l1 = createLinkedList({0});
    l2 = createLinkedList({0});
    result = addTwoNumbers(l1, l2);
    std::cout << "Test 2: ";
    printLinkedList(result);
    deleteLinkedList(result);

    // Test case 3: [9,9,9,9,9,9,9] + [9,9,9,9] = [8,9,9,9,0,0,0,1]
    l1 = createLinkedList({9, 9, 9, 9, 9, 9, 9});
    l2 = createLinkedList({9, 9, 9, 9});
    result = addTwoNumbers(l1, l2);
    std::cout << "Test 3: ";
    printLinkedList(result);
    deleteLinkedList(result);

    return 0;
}
