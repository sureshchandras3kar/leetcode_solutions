class DLLNode {
    constructor(key = 0, value = 0) {
        this.key = key;
        this.value = value;
        this.prev = null;
        this.next = null;
    }
}

class LRUCache {
    constructor(capacity) {
        this.capacity = capacity;
        this.cache = new Map();
        this.head = new DLLNode();
        this.tail = new DLLNode();
        this.head.next = this.tail;
        this.tail.prev = this.head;
    }

    get(key) {
        if (!this.cache.has(key)) {
            return -1;
        }
        const node = this.cache.get(key);
        this.moveToHead(node);
        return node.value;
    }

    put(key, value) {
        if (this.cache.has(key)) {
            const node = this.cache.get(key);
            node.value = value;
            this.moveToHead(node);
        } else {
            if (this.cache.size >= this.capacity) {
                this.removeTail();
            }
            const node = new DLLNode(key, value);
            this.cache.set(key, node);
            this.addToHead(node);
        }
    }

    moveToHead(node) {
        this.removeNode(node);
        this.addToHead(node);
    }

    removeNode(node) {
        node.prev.next = node.next;
        node.next.prev = node.prev;
    }

    addToHead(node) {
        node.next = this.head.next;
        node.prev = this.head;
        this.head.next.prev = node;
        this.head.next = node;
    }

    removeTail() {
        const node = this.tail.prev;
        this.removeNode(node);
        this.cache.delete(node.key);
    }
}

const cache = new LRUCache(2);
cache.put(1, 1);
cache.put(2, 2);
console.log(cache.get(1));  // 1
cache.put(3, 3);
console.log(cache.get(2));  // -1
cache.put(4, 4);
console.log(cache.get(1));  // -1
console.log(cache.get(3));  // 3
console.log(cache.get(4));  // 4
