import java.util.*;

/**
 * Find minimum number of coins needed to make amount using BFS.
 *
 * Time Complexity: O(n * amount)
 * Space Complexity: O(amount)
 */
public class CoinChangeBFS {
    static class State {
        int amount;
        int coins;

        State(int amount, int coins) {
            this.amount = amount;
            this.coins = coins;
        }
    }

    public static int coinChangeBFS(int[] coins, int amount) {
        if (amount == 0) return 0;

        Queue<State> queue = new LinkedList<>();
        Set<Integer> visited = new HashSet<>();

        queue.add(new State(amount, 0));
        visited.add(amount);

        while (!queue.isEmpty()) {
            State curr = queue.poll();

            for (int coin : coins) {
                int nextAmount = curr.amount - coin;
                if (nextAmount == 0) {
                    return curr.coins + 1;
                }
                if (nextAmount > 0 && !visited.contains(nextAmount)) {
                    visited.add(nextAmount);
                    queue.add(new State(nextAmount, curr.coins + 1));
                }
            }
        }

        return -1;
    }

    public static void main(String[] args) {
        System.out.println(coinChangeBFS(new int[]{1, 2, 5}, 5));   // 1
        System.out.println(coinChangeBFS(new int[]{2}, 3));         // -1
    }
}
