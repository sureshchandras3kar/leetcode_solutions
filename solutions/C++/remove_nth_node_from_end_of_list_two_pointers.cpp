#include <iostream>
#include <vector>
using namespace std;

struct ListNode {
    int val;
    ListNode *next;
    ListNode(int x = 0, ListNode *n = nullptr) : val(x), next(n) {}
};

ListNode *remove_nth_node_two_pointers(ListNode *head, int n) {
    /**
     * Remove the nth node from the end of a linked list using two pointers in one
     * pass.
     *
     * Approach:
     * 1. Create a dummy node pointing to head (handles edge case of removing head)
     * 2. Use fast and slow pointers with a gap of n nodes between them
     * 3. Move both pointers until fast reaches the end
     * 4. Skip the target node by adjusting the slow pointer
     *
     * Time: O(L) single pass
     * Space: O(1)
     */

    // Create a dummy node to handle edge case of removing the head
    ListNode *dummy = new ListNode(0);
    dummy->next = head;
    ListNode *slow = dummy;
    ListNode *fast = dummy;

    // Move fast pointer n+1 steps ahead
    for (int i = 0; i <= n; i++) {
        if (!fast)
            return head;
        fast = fast->next;
    }

    // Move both pointers until fast reaches the end
    while (fast) {
        slow = slow->next;
        fast = fast->next;
    }

    // Remove the nth node
    ListNode *temp = slow->next;
    slow->next = slow->next->next;
    delete temp;
    ListNode *result = dummy->next;
    delete dummy;
    return result;
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
    ListNode *result1 = remove_nth_node_two_pointers(head1, 2);
    print_vector(linked_list_to_vector(result1));  // [1, 2, 3, 5]

    // Test 2: [1], n=1
    vector<int> vals2 = {1};
    ListNode *head2 = create_linked_list(vals2);
    ListNode *result2 = remove_nth_node_two_pointers(head2, 1);
    print_vector(linked_list_to_vector(result2));  // []

    // Test 3: [1, 2], n=2
    vector<int> vals3 = {1, 2};
    ListNode *head3 = create_linked_list(vals3);
    ListNode *result3 = remove_nth_node_two_pointers(head3, 2);
    print_vector(linked_list_to_vector(result3));  // [2]

    // Test 4: [1, 2], n=1
    vector<int> vals4 = {1, 2};
    ListNode *head4 = create_linked_list(vals4);
    ListNode *result4 = remove_nth_node_two_pointers(head4, 1);
    print_vector(linked_list_to_vector(result4));  // [1]

    return 0;
}
