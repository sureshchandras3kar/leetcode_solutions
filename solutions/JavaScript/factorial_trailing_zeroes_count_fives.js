function factorialTrailingZeroesCountFives(n) {
    let count = 0;
    let powerOf5 = 5;
    while (powerOf5 <= n) {
        count += Math.floor(n / powerOf5);
        powerOf5 *= 5;
    }
    return count;
}

console.log(factorialTrailingZeroesCountFives(5));  // 1
console.log(factorialTrailingZeroesCountFives(25));  // 6
