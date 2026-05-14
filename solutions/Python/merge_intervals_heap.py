from typing import List
import heapq

def merge(intervals: List[List[int]]) -> List[List[int]]:
    """Heap approach: O(n log n) time, O(n) space"""
    if not intervals:
        return []
    heap = []
    for start, end in intervals:
        heapq.heappush(heap, (start, end))
    merged = []
    while heap:
        start, end = heapq.heappop(heap)
        if merged and start <= merged[-1][1]:
            merged[-1][1] = max(merged[-1][1], end)
        else:
            merged.append([start, end])
    return merged

print(merge([[1, 3], [2, 6], [8, 10]]))
