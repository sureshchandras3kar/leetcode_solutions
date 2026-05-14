function minSubArrayLenBinarySearch(target, nums) {
    const prefix = [0];
    for (let num of nums) {
        prefix.push(prefix[prefix.length - 1] + num);
    }

    let minLength = Infinity;

    for (let right = 1; right < prefix.length; right++) {
        const needed = prefix[right] - target;
        const left = binarySearchRightmost(prefix, needed, 0, right) - 1;

        if (left >= 0 && left < right) {
            minLength = Math.min(minLength, right - left);
        }
    }

    return minLength === Infinity ? 0 : minLength;
}

function binarySearchRightmost(arr, target, lo, hi) {
    while (lo < hi) {
        const mid = Math.floor(lo + (hi - lo) / 2);
        if (arr[mid] <= target) {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    return lo;
}

const nums = [2, 3, 1, 2, 4, 3];
const target = 7;
const result = minSubArrayLenBinarySearch(target, nums);
console.log(result);  // 2
