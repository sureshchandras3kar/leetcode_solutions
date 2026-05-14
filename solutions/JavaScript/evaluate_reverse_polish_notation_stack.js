/**
 * Evaluate Reverse Polish Notation using a stack.
 *
 * Time: O(n) - process each token once
 * Space: O(n) - stack stores operands
 *
 * @param {string[]} tokens - Array of tokens representing RPN expression
 * @returns {number} - Result of evaluating the RPN expression
 */
function evaluateRpnStack(tokens) {
    const stack = [];
    const operators = new Set(['+', '-', '*', '/']);

    for (const token of tokens) {
        if (operators.has(token)) {
            // Pop two operands (order matters for - and /)
            const b = stack.pop();
            const a = stack.pop();

            let result;
            switch (token) {
                case '+':
                    result = a + b;
                    break;
                case '-':
                    result = a - b;
                    break;
                case '*':
                    result = a * b;
                    break;
                case '/':
                    // Truncate towards zero
                    result = Math.trunc(a / b);
                    break;
            }
            stack.push(result);
        } else {
            // It's a number
            stack.push(parseInt(token, 10));
        }
    }

    return stack[0];
}

// Test cases
console.log(evaluateRpnStack(["2", "1", "+", "3", "*"])); // 9
console.log(evaluateRpnStack(["4", "13", "5", "/", "+"])); // 6
console.log(evaluateRpnStack(["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"])); // 22
console.log(evaluateRpnStack(["3", "4", "+"])); // 7
console.log(evaluateRpnStack(["3", "4", "*"])); // 12

module.exports = evaluateRpnStack;
