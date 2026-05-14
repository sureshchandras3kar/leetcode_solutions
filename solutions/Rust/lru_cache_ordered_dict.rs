use std::collections::HashMap;

struct LRUCache {
    capacity: usize,
    cache: HashMap<i32, i32>,
    order: Vec<i32>,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            capacity: capacity as usize,
            cache: HashMap::new(),
            order: Vec::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(&value) = self.cache.get(&key) {
            if let Some(pos) = self.order.iter().position(|&k| k == key) {
                self.order.remove(pos);
                self.order.push(key);
            }
            value
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.cache.contains_key(&key) {
            self.cache.insert(key, value);
            if let Some(pos) = self.order.iter().position(|&k| k == key) {
                self.order.remove(pos);
            }
            self.order.push(key);
        } else {
            if self.cache.len() >= self.capacity {
                if let Some(lru_key) = self.order.first().copied() {
                    self.cache.remove(&lru_key);
                    self.order.remove(0);
                }
            }
            self.cache.insert(key, value);
            self.order.push(key);
        }
    }
}

fn main() {
    let mut cache = LRUCache::new(2);
    cache.put(1, 1);
    cache.put(2, 2);
    println!("{}", cache.get(1));  // 1
    cache.put(3, 3);
    println!("{}", cache.get(2));  // -1
    cache.put(4, 4);
    println!("{}", cache.get(1));  // -1
    println!("{}", cache.get(3));  // 3
    println!("{}", cache.get(4));  // 4
}
