#include <vector>
#include <stack>
#include <string>
#include <iostream>
#include <cmath>

using namespace std;

/**
 * Evaluate Reverse Polish Notation using a stack.
 *
 * Time: O(n) - process each token once
 * Space: O(n) - stack stores operands
 */
int evaluateRpnStack(vector<string>& tokens) {
    stack<long long> st;

    for (const string& token : tokens) {
        if (token == "+" || token == "-" || token == "*" || token == "/") {
            // Pop two operands (order matters for - and /)
            long long b = st.top();
            st.pop();
            long long a = st.top();
            st.pop();

            long long result;
            if (token == "+") {
                result = a + b;
            } else if (token == "-") {
                result = a - b;
            } else if (token == "*") {
                result = a * b;
            } else {  // token == "/"
                // Truncate towards zero
                result = (long long)(a / b);
            }
            st.push(result);
        } else {
            // It's a number
            st.push(stoll(token));
        }
    }

    return (int)st.top();
}

// Test cases
int main() {
    vector<string> test1 = {"2", "1", "+", "3", "*"};
    cout << evaluateRpnStack(test1) << endl;  // 9

    vector<string> test2 = {"4", "13", "5", "/", "+"};
    cout << evaluateRpnStack(test2) << endl;  // 6

    vector<string> test3 = {"10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"};
    cout << evaluateRpnStack(test3) << endl;  // 22

    vector<string> test4 = {"3", "4", "+"};
    cout << evaluateRpnStack(test4) << endl;  // 7

    vector<string> test5 = {"3", "4", "*"};
    cout << evaluateRpnStack(test5) << endl;  // 12

    return 0;
}
