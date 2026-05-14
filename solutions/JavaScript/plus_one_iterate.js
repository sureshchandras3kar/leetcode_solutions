function plusOneIterate(digits) {
    let carry = 1;
    for (let i = digits.length - 1; i >= 0; i--) {
        digits[i] += carry;
        if (digits[i] < 10) {
            return digits;
        }
        digits[i] = 0;
    }

    return [1, ...digits];
}

console.log(plusOneIterate([1, 2, 3]));  // [1, 2, 4]
console.log(plusOneIterate([9, 9, 9]));  // [1, 0, 0, 0]
