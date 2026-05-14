function addBinaryIterative(a, b) {
    let result = "";
    let carry = 0;
    let i = a.length - 1;
    let j = b.length - 1;

    while (i >= 0 || j >= 0 || carry) {
        const digitA = i >= 0 ? parseInt(a[i]) : 0;
        const digitB = j >= 0 ? parseInt(b[j]) : 0;
        const total = digitA + digitB + carry;
        result = (total % 2) + result;
        carry = Math.floor(total / 2);
        i--;
        j--;
    }

    return result;
}

console.log(addBinaryIterative("11", "1"));  // "100"
console.log(addBinaryIterative("1010", "1011"));  // "10101"
