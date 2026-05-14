class ListNode {
    constructor(val = 0, next = null) {
        this.val = val;
        this.next = next;
    }
}

class MinHeap {
    constructor() {
        this.heap = [];
    }
    
    push(node) {
        this.heap.push(node);
        this.bubbleUp(this.heap.length - 1);
    }
    
    pop() {
        if (this.heap.length === 0) return null;
        const min = this.heap[0];
        this.heap[0] = this.heap[this.heap.length - 1];
        this.heap.pop();
        this.bubbleDown(0);
        return min;
    }
    
    bubbleUp(idx) {
        while (idx > 0) {
            const parent = Math.floor((idx - 1) / 2);
            if (this.heap[parent].val <= this.heap[idx].val) break;
            [this.heap[parent], this.heap[idx]] = [this.heap[idx], this.heap[parent]];
            idx = parent;
        }
    }
    
    bubbleDown(idx) {
        while (2 * idx + 1 < this.heap.length) {
            let smallest = idx;
            const left = 2 * idx + 1;
            const right = 2 * idx + 2;
            if (this.heap[left].val < this.heap[smallest].val) smallest = left;
            if (right < this.heap.length && this.heap[right].val < this.heap[smallest].val) smallest = right;
            if (smallest === idx) break;
            [this.heap[smallest], this.heap[idx]] = [this.heap[idx], this.heap[smallest]];
            idx = smallest;
        }
    }
    
    isEmpty() {
        return this.heap.length === 0;
    }
}

function mergeKLists(lists) {
    const heap = new MinHeap();
    for (let lst of lists) {
        if (lst) heap.push(lst);
    }
    
    const dummy = new ListNode(0);
    let curr = dummy;
    
    while (!heap.isEmpty()) {
        const node = heap.pop();
        curr.next = node;
        curr = curr.next;
        
        if (node.next) heap.push(node.next);
    }
    
    return dummy.next;
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
