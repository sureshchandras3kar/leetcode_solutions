function minSubArrayLenSlidingWindow(target, nums) {
    let left = 0;
    let currentSum = 0;
    let minLength = Infinity;

    for (let right = 0; right < nums.length; right++) {
        currentSum += nums[right];

        while (currentSum >= target) {
            minLength = Math.min(minLength, right - left + 1);
            currentSum -= nums[left];
            left++;
        }
    }

    return minLength === Infinity ? 0 : minLength;
}

const nums = [2, 3, 1, 2, 4, 3];
const target = 7;
const result = minSubArrayLenSlidingWindow(target, nums);
console.log(result);  // 2
