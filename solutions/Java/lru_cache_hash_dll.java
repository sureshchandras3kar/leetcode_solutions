import java.util.HashMap;

class DLLNode {
    int key;
    int value;
    DLLNode prev;
    DLLNode next;

    DLLNode(int key, int value) {
        this.key = key;
        this.value = value;
    }

    DLLNode() {
        this(0, 0);
    }
}

public class LRUCache {
    private int capacity;
    private HashMap<Integer, DLLNode> cache;
    private DLLNode head;
    private DLLNode tail;

    public LRUCache(int capacity) {
        this.capacity = capacity;
        this.cache = new HashMap<>();
        this.head = new DLLNode();
        this.tail = new DLLNode();
        head.next = tail;
        tail.prev = head;
    }

    public int get(int key) {
        if (!cache.containsKey(key)) {
            return -1;
        }
        DLLNode node = cache.get(key);
        moveToHead(node);
        return node.value;
    }

    public void put(int key, int value) {
        if (cache.containsKey(key)) {
            DLLNode node = cache.get(key);
            node.value = value;
            moveToHead(node);
        } else {
            if (cache.size() >= capacity) {
                removeTail();
            }
            DLLNode node = new DLLNode(key, value);
            cache.put(key, node);
            addToHead(node);
        }
    }

    private void moveToHead(DLLNode node) {
        removeNode(node);
        addToHead(node);
    }

    private void removeNode(DLLNode node) {
        node.prev.next = node.next;
        node.next.prev = node.prev;
    }

    private void addToHead(DLLNode node) {
        node.next = head.next;
        node.prev = head;
        head.next.prev = node;
        head.next = node;
    }

    private void removeTail() {
        DLLNode node = tail.prev;
        removeNode(node);
        cache.remove(node.key);
    }

    public static void main(String[] args) {
        LRUCache cache = new LRUCache(2);
        cache.put(1, 1);
        cache.put(2, 2);
        System.out.println(cache.get(1));  // 1
        cache.put(3, 3);
        System.out.println(cache.get(2));  // -1
        cache.put(4, 4);
        System.out.println(cache.get(1));  // -1
        System.out.println(cache.get(3));  // 3
        System.out.println(cache.get(4));  // 4
    }
}
