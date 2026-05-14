#include <iostream>
#include <vector>
#include <queue>

struct ListNode {
    int val;
    ListNode* next;
    ListNode(int x) : val(x), next(nullptr) {}
};

struct Compare {
    bool operator()(ListNode* a, ListNode* b) {
        return a->val > b->val;
    }
};

ListNode* mergeKLists(std::vector<ListNode*>& lists) {
    ListNode dummy(0);
    ListNode* curr = &dummy;
    
    std::priority_queue<ListNode*, std::vector<ListNode*>, Compare> pq;
    for (ListNode* lst : lists) {
        if (lst) pq.push(lst);
    }
    
    while (!pq.empty()) {
        ListNode* node = pq.top();
        pq.pop();
        curr->next = node;
        curr = curr->next;
        
        if (node->next) pq.push(node->next);
    }
    
    return dummy.next;
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
