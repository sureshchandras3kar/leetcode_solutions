from typing import List

def insert(intervals: List[List[int]], newInterval: List[int]) -> List[List[int]]:
    """Greedy approach: O(n) time, O(n) space"""
    result, i, new_start, new_end = [], 0, newInterval[0], newInterval[1]
    while i < len(intervals) and intervals[i][1] < new_start:
        result.append(intervals[i])
        i += 1
    while i < len(intervals) and intervals[i][0] <= new_end:
        new_start = min(new_start, intervals[i][0])
        new_end = max(new_end, intervals[i][1])
        i += 1
    result.append([new_start, new_end])
    while i < len(intervals):
        result.append(intervals[i])
        i += 1
    return result
