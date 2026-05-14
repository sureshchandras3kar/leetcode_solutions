function powxnFastExponentiation(x, n) {
    if (n === 0) return 1.0;

    let N = n;
    if (N < 0) {
        x = 1 / x;
        N = -N;
    }

    let result = 1.0;
    while (N > 0) {
        if (N % 2 === 1) {
            result *= x;
        }
        x *= x;
        N = Math.floor(N / 2);
    }

    return result;
}

console.log(powxnFastExponentiation(2.0, 10));  // 1024.0
console.log(powxnFastExponentiation(2.1, 3));  // 9.261
