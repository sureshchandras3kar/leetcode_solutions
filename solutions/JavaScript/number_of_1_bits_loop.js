function numberOf1BitsLoop(n) {
    let count = 0;
    while (n) {
        count += n & 1;
        n >>>= 1;
    }
    return count;
}

console.log(numberOf1BitsLoop(11));  // 3
console.log(numberOf1BitsLoop(128));  // 1
