/**
 * Bit operations approach - convert to BigInt, add, convert back to binary.
 * Time: O(max(a.length, b.length))
 * Space: O(max(a.length, b.length)) for result
 */
function addBinaryBitOperations(a, b) {
    const numA = BigInt(a, 2);  // Can't use radix in BigInt constructor
    const numB = BigInt(b, 2);

    // Use parseInt to convert
    const totalNum = BigInt(parseInt(a, 2) + parseInt(b, 2));
    return totalNum.toString(2);
}

// Alternative simpler approach
function addBinaryBitOperationsSimple(a, b) {
    return (parseInt(a, 2) + parseInt(b, 2)).toString(2);
}

console.log(addBinaryBitOperationsSimple("11", "1"));      // "100"
console.log(addBinaryBitOperationsSimple("1010", "1011")); // "10101"
console.log(addBinaryBitOperationsSimple("0", "0"));       // "0"
