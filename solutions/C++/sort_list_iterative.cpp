#include <iostream>

struct ListNode {
    int val;
    ListNode* next;
    ListNode(int x) : val(x), next(nullptr) {}
};

ListNode* merge(ListNode* l1, ListNode* l2) {
    ListNode dummy(0);
    ListNode* curr = &dummy;
    while (l1 && l2) {
        if (l1->val <= l2->val) {
            curr->next = l1;
            l1 = l1->next;
        } else {
            curr->next = l2;
            l2 = l2->next;
        }
        curr = curr->next;
    }
    curr->next = l1 ? l1 : l2;
    return dummy.next;
}

ListNode* sortList(ListNode* head) {
    if (!head) return head;
    
    ListNode dummy(0);
    dummy.next = head;
    ListNode* dummy_ptr = &dummy;
    
    int length = 0;
    ListNode* curr = head;
    while (curr) {
        length++;
        curr = curr->next;
    }
    
    for (int size = 1; size < length; size *= 2) {
        ListNode* prev = dummy_ptr;
        curr = dummy_ptr->next;
        
        while (curr) {
            ListNode* l1 = curr;
            ListNode* l1_tail = l1;
            int l1_len = 0;
            while (l1_len < size && l1_tail) {
                l1_tail = l1_tail->next;
                l1_len++;
            }
            
            ListNode* l2 = l1_tail;
            int l2_len = 0;
            while (l2_len < size && l2) {
                l2 = l2->next;
                l2_len++;
            }
            
            l1_tail = l1;
            for (int i = 1; i < l1_len; i++) {
                if (l1_tail) l1_tail = l1_tail->next;
            }
            if (l1_tail) l1_tail->next = nullptr;
            
            prev->next = merge(l1, l2);
            while (prev->next) prev = prev->next;
            curr = l2;
        }
    }
    
    return dummy_ptr->next;
}

int main() {
    ListNode* head = new ListNode(4);
    head->next = new ListNode(2);
    head->next->next = new ListNode(1);
    head->next->next->next = new ListNode(3);
    
    ListNode* result = sortList(head);
    while (result) {
        std::cout << result->val << " ";
        result = result->next;
    }
    std::cout << std::endl;
    return 0;
}
