#include <iostream>
#include <vector>
using namespace std;

struct ListNode {
    int val;
    ListNode *next;
    ListNode(int x = 0, ListNode *n = nullptr) : val(x), next(n) {}
};

ListNode *rotate_list_circular(ListNode *head, int k) {
    /**
     * Rotate a linked list to the right by k positions using the circular approach.
     *
     * Approach:
     * 1. Calculate the effective rotation: k = k % length
     * 2. Create a circular list by connecting the tail to the head
     * 3. Find the new head position: walk (length - k) steps from the original head
     * 4. Break the circle at the appropriate point
     *
     * Time: O(n) - single pass to find length and establish circle, walk (length - k) steps
     * Space: O(1) - only using pointers
     */

    if (!head || !head->next || k == 0) {
        return head;
    }

    // Step 1: Find the length of the list and the tail
    int length = 0;
    ListNode *tail = head;
    while (tail->next) {
        length++;
        tail = tail->next;
    }
    length++;  // Add 1 for the tail node itself

    // Step 2: Normalize k
    k = k % length;
    if (k == 0) {
        return head;
    }

    // Step 3: Create a circular list
    tail->next = head;

    // Step 4: Find the new head position
    // After rotation by k, the new head is at position (length - k)
    // We need to walk (length - k) steps from the original head
    int steps_to_walk = length - k;
    ListNode *current = head;
    for (int i = 0; i < steps_to_walk - 1; i++) {
        current = current->next;
    }

    // Step 5: Break the circle
    ListNode *new_head = current->next;
    current->next = nullptr;

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
    ListNode *result1 = rotate_list_circular(head1, 2);
    print_vector(linked_list_to_vector(result1));  // [4, 5, 1, 2, 3]

    // Test 2: [0, 1, 2], k=4
    vector<int> vals2 = {0, 1, 2};
    ListNode *head2 = create_linked_list(vals2);
    ListNode *result2 = rotate_list_circular(head2, 4);
    print_vector(linked_list_to_vector(result2));  // [2, 0, 1]

    // Test 3: [1], k=1
    vector<int> vals3 = {1};
    ListNode *head3 = create_linked_list(vals3);
    ListNode *result3 = rotate_list_circular(head3, 1);
    print_vector(linked_list_to_vector(result3));  // [1]

    // Test 4: [1, 2], k=2
    vector<int> vals4 = {1, 2};
    ListNode *head4 = create_linked_list(vals4);
    ListNode *result4 = rotate_list_circular(head4, 2);
    print_vector(linked_list_to_vector(result4));  // [1, 2]

    // Test 5: [1, 2, 3, 4, 5], k=0
    vector<int> vals5 = {1, 2, 3, 4, 5};
    ListNode *head5 = create_linked_list(vals5);
    ListNode *result5 = rotate_list_circular(head5, 0);
    print_vector(linked_list_to_vector(result5));  // [1, 2, 3, 4, 5]

    return 0;
}
