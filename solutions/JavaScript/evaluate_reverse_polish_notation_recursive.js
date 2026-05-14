/**
 * Evaluate Reverse Polish Notation using recursion.
 *
 * Time: O(n) - process each token once
 * Space: O(n) - recursion stack depth
 *
 * @param {string[]} tokens - Array of tokens representing RPN expression
 * @returns {number} - Result of evaluating the RPN expression
 */
function evaluateRpnRecursive(tokens) {
    let index = tokens.length - 1;
    const operators = new Set(['+', '-', '*', '/']);

    function helper() {
        const token = tokens[index--];

        if (operators.has(token)) {
            // Process in reverse order for recursion (right operand first)
            const b = helper();
            const a = helper();

            switch (token) {
                case '+':
                    return a + b;
                case '-':
                    return a - b;
                case '*':
                    return a * b;
                case '/':
                    return Math.trunc(a / b);
            }
        } else {
            return parseInt(token, 10);
        }
    }

    return helper();
}

// Test cases
console.log(evaluateRpnRecursive(["2", "1", "+", "3", "*"])); // 9
console.log(evaluateRpnRecursive(["4", "13", "5", "/", "+"])); // 6
console.log(evaluateRpnRecursive(["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"])); // 22
console.log(evaluateRpnRecursive(["3", "4", "+"])); // 7
console.log(evaluateRpnRecursive(["3", "4", "*"])); // 12

module.exports = evaluateRpnRecursive;
