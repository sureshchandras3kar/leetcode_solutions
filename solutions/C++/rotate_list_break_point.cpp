#include <iostream>
#include <vector>
using namespace std;

struct ListNode {
    int val;
    ListNode *next;
    ListNode(int x = 0, ListNode *n = nullptr) : val(x), next(n) {}
};

ListNode *rotate_list_break_point(ListNode *head, int k) {
    /**
     * Rotate a linked list to the right by k positions using the break point approach.
     *
     * Approach:
     * 1. Calculate the effective rotation: k = k % length
     * 2. Find the break point: the node at position (length - k - 1)
     * 3. Perform rotation by breaking the list at the break point
     *
     * Time: O(n) - single pass to find length, single pass to find break point
     * Space: O(1) - only using pointers
     */

    if (!head || !head->next || k == 0) {
        return head;
    }

    // Step 1: Find the length of the list
    int length = 0;
    ListNode *current = head;
    while (current) {
        length++;
        current = current->next;
    }

    // Step 2: Normalize k (handle cases where k > length)
    k = k % length;
    if (k == 0) {
        return head;
    }

    // Step 3: Find the break point (node before rotation point)
    // We need to find the (length - k - 1)th node
    current = head;
    for (int i = 0; i < length - k - 1; i++) {
        current = current->next;
    }

    // Step 4: Perform rotation
    // The new head is current->next
    ListNode *new_head = current->next;
    current->next = nullptr;

    // Find the tail of the new list and connect it back to the old head
    ListNode *tail = new_head;
    while (tail->next) {
        tail = tail->next;
    }
    tail->next = head;

    return new_head;
}

// Helper function to create a linked list from a vector
ListNode *create_linked_list(vector<int> &values) {
    if (values.empty())
        return nullptr;
    ListNode *head = new ListNode(values[0]);
    ListNode *current = head;
    for (int i = 1; i < values.size(); i++) {
        current->next = new ListNode(values[i]);
        current = current->next;
    }
    return head;
}

// Helper function to convert linked list to vector for easy printing
vector<int> linked_list_to_vector(ListNode *head) {
    vector<int> result;
    ListNode *current = head;
    while (current) {
        result.push_back(current->val);
        current = current->next;
    }
    return result;
}

// Helper function to print a vector
void print_vector(const vector<int> &v) {
    cout << "[";
    for (int i = 0; i < v.size(); i++) {
        cout << v[i];
        if (i < v.size() - 1)
            cout << ", ";
    }
    cout << "]" << endl;
}

// Test cases
int main() {
    // Test 1: [1, 2, 3, 4, 5], k=2
    vector<int> vals1 = {1, 2, 3, 4, 5};
    ListNode *head1 = create_linked_list(vals1);
    ListNode *result1 = rotate_list_break_point(head1, 2);
    print_vector(linked_list_to_vector(result1));  // [4, 5, 1, 2, 3]

    // Test 2: [0, 1, 2], k=4
    vector<int> vals2 = {0, 1, 2};
    ListNode *head2 = create_linked_list(vals2);
    ListNode *result2 = rotate_list_break_point(head2, 4);
    print_vector(linked_list_to_vector(result2));  // [2, 0, 1]

    // Test 3: [1], k=1
    vector<int> vals3 = {1};
    ListNode *head3 = create_linked_list(vals3);
    ListNode *result3 = rotate_list_break_point(head3, 1);
    print_vector(linked_list_to_vector(result3));  // [1]

    // Test 4: [1, 2], k=2
    vector<int> vals4 = {1, 2};
    ListNode *head4 = create_linked_list(vals4);
    ListNode *result4 = rotate_list_break_point(head4, 2);
    print_vector(linked_list_to_vector(result4));  // [1, 2]

    // Test 5: [1, 2, 3, 4, 5], k=0
    vector<int> vals5 = {1, 2, 3, 4, 5};
    ListNode *head5 = create_linked_list(vals5);
    ListNode *result5 = rotate_list_break_point(head5, 0);
    print_vector(linked_list_to_vector(result5));  // [1, 2, 3, 4, 5]

    return 0;
}
