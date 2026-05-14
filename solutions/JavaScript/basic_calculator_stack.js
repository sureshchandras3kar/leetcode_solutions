/**
 * Evaluate a mathematical expression with +, -, *, /, and parentheses.
 *
 * Approach: Recursive descent parser
 * - Parse expression level (+ and -)
 * - Parse term level (* and /)
 * - Parse factor level (numbers and parentheses)
 * - Respects operator precedence naturally
 *
 * Time: O(n) - single pass through string
 * Space: O(d) - recursion depth equals nesting level
 */

class BasicCalculatorStack {
    constructor(s) {
        this.s = s;
        this.pos = 0;
    }

    parseNumber() {
        while (this.pos < this.s.length && /\s/.test(this.s[this.pos])) this.pos++;
        let num = 0;
        while (this.pos < this.s.length && /\d/.test(this.s[this.pos])) {
            num = num * 10 + parseInt(this.s[this.pos]);
            this.pos++;
        }
        return num;
    }

    parseFactor() {
        while (this.pos < this.s.length && /\s/.test(this.s[this.pos])) this.pos++;
        if (this.s[this.pos] === '(') {
            this.pos++;  // Skip '('
            let result = this.parseExpression();
            while (this.pos < this.s.length && /\s/.test(this.s[this.pos])) this.pos++;
            this.pos++;  // Skip ')'
            return result;
        } else {
            return this.parseNumber();
        }
    }

    parseTerm() {
        let result = this.parseFactor();
        while (this.pos < this.s.length) {
            while (this.pos < this.s.length && /\s/.test(this.s[this.pos])) this.pos++;
            if (this.pos >= this.s.length || !['*', '/'].includes(this.s[this.pos])) break;
            const op = this.s[this.pos];
            this.pos++;
            const operand = this.parseFactor();
            if (op === '*') {
                result *= operand;
            } else {
                result = Math.floor(result / operand);
            }
        }
        return result;
    }

    parseExpression() {
        let result = this.parseTerm();
        while (this.pos < this.s.length) {
            while (this.pos < this.s.length && /\s/.test(this.s[this.pos])) this.pos++;
            if (this.pos >= this.s.length || !['+', '-'].includes(this.s[this.pos])) break;
            const op = this.s[this.pos];
            this.pos++;
            const operand = this.parseTerm();
            if (op === '+') {
                result += operand;
            } else {
                result -= operand;
            }
        }
        return result;
    }

    calculate() {
        return this.parseExpression();
    }
}

function basicCalculatorStack(s) {
    return new BasicCalculatorStack(s).calculate();
}

// Test cases
console.log(basicCalculatorStack("1 + 1"));  // 2
console.log(basicCalculatorStack(" 2-1 + 2 "));  // 3
console.log(basicCalculatorStack("(1+(4+5+2)-3)+(6+8)"));  // 23
console.log(basicCalculatorStack("2*(5+5*2)/3+(6/2*5)"));  // 25

module.exports = basicCalculatorStack;
