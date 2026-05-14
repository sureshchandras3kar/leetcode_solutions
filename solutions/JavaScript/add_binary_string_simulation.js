/**
 * String simulation approach - simulate binary addition from right to left.
 * Time: O(max(a.length, b.length))
 * Space: O(max(a.length, b.length)) for result
 */
function addBinaryStringSimulation(a, b) {
    const result = [];
    let carry = 0;
    let i = a.length - 1;
    let j = b.length - 1;

    while (i >= 0 || j >= 0 || carry) {
        const digitA = i >= 0 ? parseInt(a[i]) : 0;
        const digitB = j >= 0 ? parseInt(b[j]) : 0;

        const total = digitA + digitB + carry;
        result.push(total % 2);
        carry = Math.floor(total / 2);

        i--;
        j--;
    }

    return result.reverse().join('');
}

console.log(addBinaryStringSimulation("11", "1"));      // "100"
console.log(addBinaryStringSimulation("1010", "1011")); // "10101"
console.log(addBinaryStringSimulation("0", "0"));       // "0"
