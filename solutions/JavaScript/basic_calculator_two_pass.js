/**
 * Evaluate a mathematical expression with +, -, *, /, and parentheses.
 *
 * Approach: Two-Pass with precedence handling
 * - First pass: convert infix to postfix notation (Reverse Polish Notation)
 * - Second pass: evaluate postfix expression using a stack
 * - Handles operator precedence (* and / before + and -)
 *
 * Time: O(n) - two passes through string
 * Space: O(n) - stacks for postfix and evaluation
 */

function basicCalculatorTwoPass(s) {
    if (!s) return 0;

    const precedence = (op) => {
        if (op === '+' || op === '-') return 1;
        if (op === '*' || op === '/') return 2;
        return 0;
    };

    // Tokenize the expression
    const tokens = [];
    let i = 0;
    while (i < s.length) {
        if (/\s/.test(s[i])) {
            i++;
        } else if (/\d/.test(s[i])) {
            let num = 0;
            while (i < s.length && /\d/.test(s[i])) {
                num = num * 10 + parseInt(s[i]);
                i++;
            }
            tokens.push(num);
        } else if ('+-*/()'.includes(s[i])) {
            tokens.push(s[i]);
            i++;
        } else {
            i++;
        }
    }

    // Convert infix to postfix (Shunting Yard algorithm)
    const output = [];
    const opStack = [];

    for (const token of tokens) {
        if (typeof token === 'number') {
            output.push(token);
        } else if (token === '(') {
            opStack.push(token);
        } else if (token === ')') {
            while (opStack.length > 0 && opStack[opStack.length - 1] !== '(') {
                output.push(opStack.pop());
            }
            if (opStack.length > 0) {
                opStack.pop();  // Remove '('
            }
        } else if ('+-*/'.includes(token)) {
            while (opStack.length > 0 &&
                   opStack[opStack.length - 1] !== '(' &&
                   precedence(opStack[opStack.length - 1]) >= precedence(token)) {
                output.push(opStack.pop());
            }
            opStack.push(token);
        }
    }

    while (opStack.length > 0) {
        output.push(opStack.pop());
    }

    // Evaluate postfix expression
    const evalStack = [];
    for (const token of output) {
        if (typeof token === 'number') {
            evalStack.push(token);
        } else {
            const b = evalStack.pop();
            const a = evalStack.pop();
            switch (token) {
                case '+':
                    evalStack.push(a + b);
                    break;
                case '-':
                    evalStack.push(a - b);
                    break;
                case '*':
                    evalStack.push(a * b);
                    break;
                case '/':
                    evalStack.push(Math.floor(a / b));
                    break;
            }
        }
    }

    return evalStack.length > 0 ? evalStack[0] : 0;
}

// Test cases
console.log(basicCalculatorTwoPass("1 + 1"));  // 2
console.log(basicCalculatorTwoPass(" 2-1 + 2 "));  // 3
console.log(basicCalculatorTwoPass("(1+(4+5+2)-3)+(6+8)"));  // 23
console.log(basicCalculatorTwoPass("2*(5+5*2)/3+(6/2*5)"));  // 17

module.exports = basicCalculatorTwoPass;
