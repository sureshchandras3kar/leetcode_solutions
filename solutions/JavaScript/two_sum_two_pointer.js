function twoSumTwoPointer(nums, target) {
    const sorted = [...nums].sort((a, b) => a - b);
    let left = 0, right = sorted.length - 1;
    while (left < right) {
        const currentSum = sorted[left] + sorted[right];
        if (currentSum === target) {
            return [sorted[left], sorted[right]];
        } else if (currentSum < target) {
            left++;
        } else {
            right--;
        }
    }
    return [];
}

const nums = [2, 7, 11, 15];
const target = 9;
const result = twoSumTwoPointer(nums, target);
console.log(result);
