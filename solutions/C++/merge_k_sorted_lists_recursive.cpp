#include <iostream>
#include <vector>

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

ListNode* partition(std::vector<ListNode*>& lists, int left, int right) {
    if (left == right) return lists[left];
    if (left > right) return nullptr;
    
    int mid = left + (right - left) / 2;
    ListNode* l1 = partition(lists, left, mid);
    ListNode* l2 = partition(lists, mid + 1, right);
    return merge(l1, l2);
}

ListNode* mergeKLists(std::vector<ListNode*>& lists) {
    if (lists.empty()) return nullptr;
    return partition(lists, 0, lists.size() - 1);
}

int main() {
    ListNode* l1 = new ListNode(1);
    l1->next = new ListNode(4);
    l1->next->next = new ListNode(5);
    
    ListNode* l2 = new ListNode(1);
    l2->next = new ListNode(3);
    l2->next->next = new ListNode(4);
    
    ListNode* l3 = new ListNode(2);
    l3->next = new ListNode(6);
    
    std::vector<ListNode*> lists = {l1, l2, l3};
    ListNode* result = mergeKLists(lists);
    while (result) {
        std::cout << result->val << " ";
        result = result->next;
    }
    std::cout << std::endl;
    return 0;
}
