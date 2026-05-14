#include <iostream>
#include <vector>
using namespace std;

struct ListNode {
    int val;
    ListNode *next;
    ListNode(int x = 0, ListNode *n = nullptr) : val(x), next(n) {}
};

ListNode *remove_nth_node_two_pass(ListNode *head, int n) {
    /**
     * Remove the nth node from the end of a linked list using two passes.
     *
     * Approach:
     * 1. First pass: count the total number of nodes
     * 2. Second pass: find and skip the nth node from the end
     *
     * Time: O(L) + O(L-n) = O(L)
     * Space: O(1)
     */

    // First pass: count the total nodes
    int length = 0;
    ListNode *current = head;
    while (current) {
        length++;
        current = current->next;
    }

    // Edge case: removing the head node
    if (length == n) {
        ListNode *new_head = head->next;
        delete head;
        return new_head;
    }

    // Second pass: find the node before the one to remove
    current = head;
    for (int i = 0; i < length - n - 1; i++) {
        current = current->next;
    }

    // Remove the nth node
    ListNode *temp = current->next;
    current->next = current->next->next;
    delete temp;
    return head;
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
    // Test 1: [1, 2, 3, 4, 5], n=2
    vector<int> vals1 = {1, 2, 3, 4, 5};
    ListNode *head1 = create_linked_list(vals1);
    ListNode *result1 = remove_nth_node_two_pass(head1, 2);
    print_vector(linked_list_to_vector(result1));  // [1, 2, 3, 5]

    // Test 2: [1], n=1
    vector<int> vals2 = {1};
    ListNode *head2 = create_linked_list(vals2);
    ListNode *result2 = remove_nth_node_two_pass(head2, 1);
    print_vector(linked_list_to_vector(result2));  // []

    // Test 3: [1, 2], n=2
    vector<int> vals3 = {1, 2};
    ListNode *head3 = create_linked_list(vals3);
    ListNode *result3 = remove_nth_node_two_pass(head3, 2);
    print_vector(linked_list_to_vector(result3));  // [2]

    // Test 4: [1, 2], n=1
    vector<int> vals4 = {1, 2};
    ListNode *head4 = create_linked_list(vals4);
    ListNode *result4 = remove_nth_node_two_pass(head4, 1);
    print_vector(linked_list_to_vector(result4));  // [1]

    return 0;
}
