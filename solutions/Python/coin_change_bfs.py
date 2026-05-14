from typing import List
from collections import deque


def coin_change_bfs(coins: List[int], amount: int) -> int:
    """
    Find minimum number of coins needed to make amount using BFS.

    Time Complexity: O(n * amount) where n = len(coins)
    Space Complexity: O(amount)

    Treat this as a shortest path problem in a graph:
    - Start at amount 0 (need 0 coins)
    - Each coin represents an edge to a smaller amount
    - BFS finds the shortest path to amount
    """
    if amount == 0:
        return 0

    queue = deque([(amount, 0)])  # (remaining_amount, num_coins)
    visited = {amount}

    while queue:
        current_amount, num_coins = queue.popleft()

        for coin in coins:
            next_amount = current_amount - coin
            if next_amount == 0:
                return num_coins + 1
            if next_amount > 0 and next_amount not in visited:
                visited.add(next_amount)
                queue.append((next_amount, num_coins + 1))

    return -1


if __name__ == "__main__":
    print(coin_change_bfs([1, 2, 5], 5))      # 1
    print(coin_change_bfs([2], 3))            # -1
    print(coin_change_bfs([10], 10))          # 1
    print(coin_change_bfs([1, 3, 4], 6))      # 2
