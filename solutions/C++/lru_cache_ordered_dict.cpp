#include <iostream>
#include <map>

class LRUCache {
private:
    int capacity;
    std::map<int, int> cache;
    int timestamp = 0;
    std::map<int, int> access_time;

public:
    LRUCache(int capacity) : capacity(capacity) {}

    int get(int key) {
        if (cache.find(key) == cache.end()) {
            return -1;
        }
        access_time[key] = ++timestamp;
        return cache[key];
    }

    void put(int key, int value) {
        if (cache.find(key) != cache.end()) {
            cache[key] = value;
            access_time[key] = ++timestamp;
        } else {
            if ((int)cache.size() >= capacity) {
                int lru_key = -1;
                int min_time = INT_MAX;
                for (auto& p : access_time) {
                    if (cache.find(p.first) != cache.end() && p.second < min_time) {
                        min_time = p.second;
                        lru_key = p.first;
                    }
                }
                cache.erase(lru_key);
                access_time.erase(lru_key);
            }
            cache[key] = value;
            access_time[key] = ++timestamp;
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
