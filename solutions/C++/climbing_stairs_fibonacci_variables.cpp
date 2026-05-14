#include <iostream>

using namespace std;

/**
 * Climb n stairs with Fibonacci variables approach (space-optimized).
 *
 * Time Complexity: O(n)
 * Space Complexity: O(1)
 */
int climbingStairsFibonacciVariables(int n) {
    if (n <= 1) return 1;

    int prev2 = 1;  // dp[0]
    int prev1 = 1;  // dp[1]

    for (int i = 2; i <= n; i++) {
        int current = prev1 + prev2;
        prev2 = prev1;
        prev1 = current;
    }

    return prev1;
}

int main() {
    cout << climbingStairsFibonacciVariables(3) << endl;  // 3
    cout << climbingStairsFibonacciVariables(4) << endl;  // 5
    cout << climbingStairsFibonacciVariables(5) << endl;  // 8
    return 0;
}
