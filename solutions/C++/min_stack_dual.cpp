#include <iostream>
#include <stack>
#include <algorithm>

using namespace std;

class MinStack {
private:
    stack<int> main_stack;
    stack<int> min_stack;

public:
    /**
     * Min Stack using two stacks (dual stack approach).
     *
     * Maintains a main stack for all values and a separate min stack
     * that tracks the minimum value at each level.
     *
     * Time: O(1) per operation
     * Space: O(n)
     */

    void push(int val) {
        main_stack.push(val);
        // Push to min_stack the minimum of val and current min
        if (min_stack.empty()) {
            min_stack.push(val);
        } else {
            min_stack.push(min(val, min_stack.top()));
        }
    }

    void pop() {
        main_stack.pop();
        min_stack.pop();
    }

    int top() {
        return main_stack.top();
    }

    int getMin() {
        return min_stack.top();
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
