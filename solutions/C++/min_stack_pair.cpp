#include <iostream>
#include <stack>
#include <algorithm>

using namespace std;

class MinStack {
private:
    stack<pair<int, int>> stack;

public:
    /**
     * Min Stack using single stack with (value, min) pairs.
     *
     * Each element in the stack is a pair of (value, min_at_this_level).
     * The min_at_this_level is the minimum value from the bottom up to
     * and including the current element.
     *
     * Time: O(1) per operation
     * Space: O(n)
     */

    void push(int val) {
        if (stack.empty()) {
            // First element: value is the minimum
            stack.push({val, val});
        } else {
            // Store both value and the minimum up to this point
            int current_min = min(val, stack.top().second);
            stack.push({val, current_min});
        }
    }

    void pop() {
        stack.pop();
    }

    int top() {
        return stack.top().first;
    }

    int getMin() {
        return stack.top().second;
    }
};

int main() {
    MinStack stack;
    stack.push(-2);
    stack.push(0);
    stack.push(-3);
    cout << "Min: " << stack.getMin() << endl;  // -3
    stack.pop();
    cout << "Top: " << stack.top() << endl;     // 0
    cout << "Min: " << stack.getMin() << endl;  // -2
    return 0;
}
