class ListNode {
    constructor(val = 0, next = null) {
        this.val = val;
        this.next = next;
    }
}

function mergeKLists(lists) {
    if (lists.length === 0) return null;
    
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
    
    function partition(left, right) {
        if (left === right) return lists[left];
        if (left > right) return null;
        
        const mid = Math.floor((left + right) / 2);
        const l1 = partition(left, mid);
        const l2 = partition(mid + 1, right);
        return merge(l1, l2);
    }
    
    return partition(0, lists.length - 1);
}

const l1 = new ListNode(1, new ListNode(4, new ListNode(5)));
const l2 = new ListNode(1, new ListNode(3, new ListNode(4)));
const l3 = new ListNode(2, new ListNode(6));
const result = mergeKLists([l1, l2, l3]);
let curr = result;
while (curr) {
    process.stdout.write(curr.val + " ");
    curr = curr.next;
}
console.log();
