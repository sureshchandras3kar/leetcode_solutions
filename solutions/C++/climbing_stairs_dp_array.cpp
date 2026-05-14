#include <iostream>
#include <vector>

using namespace std;

/**
 * Climb n stairs with dp array approach.
 *
 * Time Complexity: O(n)
 * Space Complexity: O(n)
 */
int climbingStairsDpArray(int n) {
    if (n <= 1) return 1;

    vector<int> dp(n + 1);
    dp[0] = 1;
    dp[1] = 1;

    for (int i = 2; i <= n; i++) {
        dp[i] = dp[i - 1] + dp[i - 2];
    }

    return dp[n];
}

int main() {
    cout << climbingStairsDpArray(3) << endl;  // 3
    cout << climbingStairsDpArray(4) << endl;  // 5
    cout << climbingStairsDpArray(5) << endl;  // 8
    return 0;
}
