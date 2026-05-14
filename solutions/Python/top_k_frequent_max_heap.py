from typing import List
from collections import Counter
import heapq

def topKFrequent(nums: List[int], k: int) -> List[int]:
    freq = Counter(nums)
    # heapq.nlargest uses a heap internally — O(n log k)
    return heapq.nlargest(k, freq.keys(), key=lambda x: freq[x])

print(topKFrequent([1, 1, 1, 2, 2, 3], 2))  # [1, 2]
print(topKFrequent([1], 1))                  # [1]
