#include <iostream>
#include <unordered_map>
#include <list>

class LRUCache {
private:
    int capacity;
    std::unordered_map<int, std::pair<int, std::list<int>::iterator>> cache;
    std::list<int> order;

public:
    LRUCache(int capacity) : capacity(capacity) {}

    int get(int key) {
        if (cache.find(key) == cache.end()) {
            return -1;
        }
        order.erase(cache[key].second);
        order.push_back(key);
        cache[key].second = std::prev(order.end());
        return cache[key].first;
    }

    void put(int key, int value) {
        if (cache.find(key) != cache.end()) {
            order.erase(cache[key].second);
            order.push_back(key);
            cache[key] = {value, std::prev(order.end())};
        } else {
            if (cache.size() >= (size_t)capacity) {
                int lru_key = order.front();
                order.pop_front();
                cache.erase(lru_key);
            }
            order.push_back(key);
            cache[key] = {value, std::prev(order.end())};
        }
    }
};

int main() {
    LRUCache cache(2);
    cache.put(1, 1);
    cache.put(2, 2);
    std::cout << cache.get(1) << std::endl;  // 1
    cache.put(3, 3);
    std::cout << cache.get(2) << std::endl;  // -1
    cache.put(4, 4);
    std::cout << cache.get(1) << std::endl;  // -1
    std::cout << cache.get(3) << std::endl;  // 3
    std::cout << cache.get(4) << std::endl;  // 4
    return 0;
}
