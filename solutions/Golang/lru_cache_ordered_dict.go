package main

import "fmt"

type LRUCache struct {
	capacity int
	cache    map[int]int
	order    []int
}

func Constructor(capacity int) LRUCache {
	return LRUCache{
		capacity: capacity,
		cache:    make(map[int]int),
		order:    make([]int, 0),
	}
}

func (c *LRUCache) Get(key int) int {
	if value, ok := c.cache[key]; ok {
		c.moveToEnd(key)
		return value
	}
	return -1
}

func (c *LRUCache) Put(key int, value int) {
	if _, ok := c.cache[key]; ok {
		c.cache[key] = value
		c.moveToEnd(key)
	} else {
		if len(c.cache) >= c.capacity {
			lruKey := c.order[0]
			delete(c.cache, lruKey)
			c.order = c.order[1:]
		}
		c.cache[key] = value
		c.order = append(c.order, key)
	}
}

func (c *LRUCache) moveToEnd(key int) {
	for i, k := range c.order {
		if k == key {
			c.order = append(c.order[:i], c.order[i+1:]...)
			c.order = append(c.order, key)
			break
		}
	}
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
