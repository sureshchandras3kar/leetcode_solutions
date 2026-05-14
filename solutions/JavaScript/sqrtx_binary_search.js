function sqrtxBinarySearch(x) {
    if (x < 2) return x;

    let left = 2, right = Math.floor(x / 2);
    while (left <= right) {
        const mid = Math.floor((left + right) / 2);
        if (mid * mid === x) {
            return mid;
        } else if (mid * mid < x) {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    return right;
}

console.log(sqrtxBinarySearch(4));  // 2
console.log(sqrtxBinarySearch(8));  // 2
