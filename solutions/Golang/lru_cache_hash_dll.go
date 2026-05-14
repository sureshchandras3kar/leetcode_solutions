package main

import "fmt"

type DLLNode struct {
	key   int
	value int
	prev  *DLLNode
	next  *DLLNode
}

type LRUCache struct {
	capacity int
	cache    map[int]*DLLNode
	head     *DLLNode
	tail     *DLLNode
}

func Constructor(capacity int) LRUCache {
	head := &DLLNode{}
	tail := &DLLNode{}
	head.next = tail
	tail.prev = head

	return LRUCache{
		capacity: capacity,
		cache:    make(map[int]*DLLNode),
		head:     head,
		tail:     tail,
	}
}

func (c *LRUCache) Get(key int) int {
	if node, ok := c.cache[key]; ok {
		c.moveToHead(node)
		return node.value
	}
	return -1
}

func (c *LRUCache) Put(key int, value int) {
	if node, ok := c.cache[key]; ok {
		node.value = value
		c.moveToHead(node)
	} else {
		if len(c.cache) >= c.capacity {
			c.removeTail()
		}
		node := &DLLNode{key: key, value: value}
		c.cache[key] = node
		c.addToHead(node)
	}
}

func (c *LRUCache) moveToHead(node *DLLNode) {
	c.removeNode(node)
	c.addToHead(node)
}

func (c *LRUCache) removeNode(node *DLLNode) {
	node.prev.next = node.next
	node.next.prev = node.prev
}

func (c *LRUCache) addToHead(node *DLLNode) {
	node.next = c.head.next
	node.prev = c.head
	c.head.next.prev = node
	c.head.next = node
}

func (c *LRUCache) removeTail() {
	node := c.tail.prev
	c.removeNode(node)
	delete(c.cache, node.key)
}

func main() {
	cache := Constructor(2)
	cache.Put(1, 1)
	cache.Put(2, 2)
	fmt.Println(cache.Get(1))  // 1
	cache.Put(3, 3)
	fmt.Println(cache.Get(2))  // -1
	cache.Put(4, 4)
	fmt.Println(cache.Get(1))  // -1
	fmt.Println(cache.Get(3))  // 3
	fmt.Println(cache.Get(4))  // 4
}
