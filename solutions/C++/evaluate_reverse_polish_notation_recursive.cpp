#include <vector>
#include <string>
#include <iostream>

using namespace std;

/**
 * Evaluate Reverse Polish Notation using recursion.
 *
 * Time: O(n) - process each token once
 * Space: O(n) - recursion stack depth
 */
class Solution {
private:
    int index;
    vector<string> tokens;

    int helper() {
        string token = tokens[index--];

        if (token == "+" || token == "-" || token == "*" || token == "/") {
            // Process in reverse order for recursion (right operand first)
            long long b = helper();
            long long a = helper();

            if (token == "+") {
                return a + b;
            } else if (token == "-") {
                return a - b;
            } else if (token == "*") {
                return a * b;
            } else {  // token == "/"
                return (long long)(a / b);
            }
        } else {
            return stoll(token);
        }
    }

public:
    int evaluateRpnRecursive(vector<string>& t) {
        tokens = t;
        index = t.size() - 1;
        return helper();
    }
};

// Test cases
int main() {
    Solution sol;

    vector<string> test1 = {"2", "1", "+", "3", "*"};
    cout << sol.evaluateRpnRecursive(test1) << endl;  // 9

    vector<string> test2 = {"4", "13", "5", "/", "+"};
    cout << sol.evaluateRpnRecursive(test2) << endl;  // 6

    vector<string> test3 = {"10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"};
    cout << sol.evaluateRpnRecursive(test3) << endl;  // 22

    vector<string> test4 = {"3", "4", "+"};
    cout << sol.evaluateRpnRecursive(test4) << endl;  // 7

    vector<string> test5 = {"3", "4", "*"};
    cout << sol.evaluateRpnRecursive(test5) << endl;  // 12

    return 0;
}
