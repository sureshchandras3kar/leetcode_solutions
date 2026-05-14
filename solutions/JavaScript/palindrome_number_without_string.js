function isPalindrome(x) {
    if (x < 0) {
        return false;
    }
    if (x === 0) {
        return true;
    }
    if (x % 10 === 0) {
        return false;
    }
    let numReversed = 0;
    let originalX = x;
    while (originalX > 0) {
        const lastDigit = originalX % 10;
        numReversed = numReversed * 10 + lastDigit;
        originalX = Math.floor(originalX / 10);
    }
    return x === numReversed;
}

console.log(isPalindrome(121));  // true
console.log(isPalindrome(-121)); // false
console.log(isPalindrome(10));   // false
