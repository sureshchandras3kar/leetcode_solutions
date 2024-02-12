function isPalindrome(x) {
    const strX = x.toString();
    return strX === strX.split('').reverse().join('');
}

console.log(isPalindrome(121));  // true
console.log(isPalindrome(-121)); // false
console.log(isPalindrome(10));   // false
