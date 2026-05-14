/**
 * Find minimum number of coins needed to make amount using BFS.
 *
 * Time Complexity: O(n * amount)
 * Space Complexity: O(amount)
 */
function coinChangeBFS(coins, amount) {
    if (amount === 0) return 0;

    const queue = [[amount, 0]];  // [remaining_amount, num_coins]
    const visited = new Set([amount]);

    while (queue.length > 0) {
        const [currAmount, numCoins] = queue.shift();

        for (const coin of coins) {
            const nextAmount = currAmount - coin;
            if (nextAmount === 0) {
                return numCoins + 1;
            }
            if (nextAmount > 0 && !visited.has(nextAmount)) {
                visited.add(nextAmount);
                queue.push([nextAmount, numCoins + 1]);
            }
        }
    }

    return -1;
}

console.log(coinChangeBFS([1, 2, 5], 5));   // 1
console.log(coinChangeBFS([2], 3));        // -1
