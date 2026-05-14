class ListNode {
    constructor(val = 0, next = null) {
        this.val = val;
        this.next = next;
    }
}

function sortList(head) {
    if (!head || !head.next) return head;
    
    let slow = head, fast = head.next, prev = null;
    while (fast && fast.next) {
        prev = slow;
        slow = slow.next;
        fast = fast.next.next;
    }
    
    if (prev) prev.next = null;
    
    const left = sortList(head);
    const right = sortList(slow);
    return merge(left, right);
}

function merge(l1, l2) {
    const dummy = new ListNode(0);
    let curr = dummy;
    while (l1 && l2) {
        if (l1.val <= l2.val) {
            curr.next = l1;
            l1 = l1.next;
        } else {
            curr.next = l2;
            l2 = l2.next;
        }
        curr = curr.next;
    }
    curr.next = l1 ? l1 : l2;
    return dummy.next;
}

const head = new ListNode(4, new ListNode(2, new ListNode(1, new ListNode(3))));
const result = sortList(head);
let curr = result;
while (curr) {
    process.stdout.write(curr.val + " ");
    curr = curr.next;
}
console.log();
