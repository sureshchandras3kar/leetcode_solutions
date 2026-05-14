from typing import List

def findMinArrowShots(points: List[List[int]]) -> int:
    """Greedy approach: O(n log n) time, O(n) space"""
    if not points:
        return 0
    points.sort(key=lambda x: x[1])
    arrows = 1
    last_end = points[0][1]
    for start, end in points[1:]:
        if start > last_end:
            arrows += 1
            last_end = end
    return arrows
