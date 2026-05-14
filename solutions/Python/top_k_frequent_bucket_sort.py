from typing import List
from collections import Counter

def topKFrequent(nums: List[int], k: int) -> List[int]:
    freq = Counter(nums)
    n = len(nums)
    buckets: List[List[int]] = [[] for _ in range(n + 1)]
    for num, count in freq.items():
        buckets[count].append(num)
    result: List[int] = []
    for i in range(n, 0, -1):
        result.extend(buckets[i])
        if len(result) >= k:
            return result[:k]
    return result[:k]

print(topKFrequent([1, 1, 1, 2, 2, 3], 2))  # [1, 2]
print(topKFrequent([1], 1))                  # [1]
