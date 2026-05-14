class ListNode {
    constructor(val = 0, next = null) {
        this.val = val;
        this.next = next;
    }
}

function sortList(head) {
    if (!head) return head;
    
    const dummy = new ListNode(0, head);
    
    let length = 0;
    let curr = head;
    while (curr) {
        length++;
        curr = curr.next;
    }
    
    for (let size = 1; size < length; size *= 2) {
        let prev = dummy;
        curr = dummy.next;
        
        while (curr) {
            let l1 = curr;
            let l1_tail = l1;
            let l1_len = 0;
            while (l1_len < size && l1_tail) {
                l1_tail = l1_tail.next;
                l1_len++;
            }
            
            let l2 = l1_tail;
            let l2_len = 0;
            while (l2_len < size && l2) {
                l2 = l2.next;
                l2_len++;
            }
            
            l1_tail = l1;
            for (let i = 1; i < l1_len; i++) {
                if (l1_tail) l1_tail = l1_tail.next;
            }
            if (l1_tail) l1_tail.next = null;
            
            prev.next = merge(l1, l2);
            while (prev.next) prev = prev.next;
            curr = l2;
        }
    }
    
    return dummy.next;
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
