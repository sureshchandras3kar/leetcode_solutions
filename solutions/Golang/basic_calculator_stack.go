package main

import (
	"unicode"
)

/*
Evaluate a mathematical expression with +, -, *, /, and parentheses.

Approach: Recursive descent parser
- Parse expression level (+ and -)
- Parse term level (* and /)
- Parse factor level (numbers and parentheses)
- Respects operator precedence naturally

Time: O(n) - single pass through string
Space: O(d) - recursion depth equals nesting level
*/

type Calculator struct {
	s   string
	pos int
}

func NewCalculator(s string) *Calculator {
	return &Calculator{s: s, pos: 0}
}

func (c *Calculator) parseNumber() int {
	for c.pos < len(c.s) && unicode.IsSpace(rune(c.s[c.pos])) {
		c.pos++
	}
	num := 0
	for c.pos < len(c.s) && unicode.IsDigit(rune(c.s[c.pos])) {
		num = num*10 + int(c.s[c.pos]-'0')
		c.pos++
	}
	return num
}

func (c *Calculator) parseFactor() int {
	for c.pos < len(c.s) && unicode.IsSpace(rune(c.s[c.pos])) {
		c.pos++
	}
	if c.s[c.pos] == '(' {
		c.pos++  // Skip '('
		result := c.parseExpression()
		for c.pos < len(c.s) && unicode.IsSpace(rune(c.s[c.pos])) {
			c.pos++
		}
		c.pos++  // Skip ')'
		return result
	}
	return c.parseNumber()
}

func (c *Calculator) parseTerm() int {
	result := c.parseFactor()
	for c.pos < len(c.s) {
		for c.pos < len(c.s) && unicode.IsSpace(rune(c.s[c.pos])) {
			c.pos++
		}
		if c.pos >= len(c.s) || (c.s[c.pos] != '*' && c.s[c.pos] != '/') {
			break
		}
		op := c.s[c.pos]
		c.pos++
		operand := c.parseFactor()
		if op == '*' {
			result *= operand
		} else {
			result /= operand
		}
	}
	return result
}

func (c *Calculator) parseExpression() int {
	result := c.parseTerm()
	for c.pos < len(c.s) {
		for c.pos < len(c.s) && unicode.IsSpace(rune(c.s[c.pos])) {
			c.pos++
		}
		if c.pos >= len(c.s) || (c.s[c.pos] != '+' && c.s[c.pos] != '-') {
			break
		}
		op := c.s[c.pos]
		c.pos++
		operand := c.parseTerm()
		if op == '+' {
			result += operand
		} else {
			result -= operand
		}
	}
	return result
}

func (c *Calculator) Calculate() int {
	return c.parseExpression()
}

func basicCalculatorStack(s string) int {
	if len(s) == 0 {
		return 0
	}
	calc := NewCalculator(s)
	return calc.Calculate()
}
