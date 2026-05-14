from typing import List
import bisect


def lengthOfLIS(nums: List[int]) -> int:
    if not nums:
        return 0

    tails = []

    for num in nums:
        pos = bisect.bisect_left(tails, num)
        if pos == len(tails):
            tails.append(num)
        else:
            tails[pos] = num

    return len(tails)
