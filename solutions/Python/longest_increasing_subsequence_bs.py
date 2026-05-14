from typing import List
import bisect


def longest_increasing_subsequence_binary_search(nums: List[int]) -> int:
    """
    Find length of longest increasing subsequence using binary search O(n log n).

    Time Complexity: O(n log n)
    Space Complexity: O(n)

    Maintain array 'tails' where tails[i] = smallest tail of all LIS of length i+1
    For each num:
    - Find position in tails using binary search
    - If position = len(tails), extend the longest subsequence
    - Otherwise, replace the value at that position (better tail found)
    """
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


if __name__ == "__main__":
    print(longest_increasing_subsequence_binary_search([10, 9, 2, 5, 3, 7, 101, 18]))  # 4
    print(longest_increasing_subsequence_binary_search([0, 1, 0, 4, 4, 4, 3, 2, 1]))   # 2
    print(longest_increasing_subsequence_binary_search([3, 1, 4, 1, 5, 9, 2, 6]))      # 4
