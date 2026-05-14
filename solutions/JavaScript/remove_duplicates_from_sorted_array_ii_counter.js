/**
 * Two Pointers with Counter Approach
 * Allow each element to appear at most twice using explicit count tracking.
 *
 * Time Complexity: O(n)
 * Space Complexity: O(1)
 *
 * @param {number[]} nums
 * @return {number}
 */
function removeDuplicates(nums) {
    if (nums.length === 0) {
        return 0;
    }

    let k = 1;
    let count = 1;
    for (let i = 1; i < nums.length; i++) {
        if (nums[i] !== nums[i - 1]) {
            // New element encountered, reset counter
            count = 1;
            nums[k] = nums[i];
            k++;
        } else if (count < 2) {
            // Same element but less than 2 occurrences, allow it
            count++;
            nums[k] = nums[i];
            k++;
        }
        // else: count == 2, skip this duplicate
    }

    return k;
}

// Test cases
console.log(removeDuplicates([1, 1, 1, 2, 2, 3]));  // 5, nums = [1, 1, 2, 2, 3, _]
console.log(removeDuplicates([0, 0, 1, 1, 1, 1, 2, 3, 3]));  // 7, nums = [0, 0, 1, 1, 2, 3, 3, _, _]
console.log(removeDuplicates([1]));  // 1, nums = [1]
console.log(removeDuplicates([1, 2]));  // 2, nums = [1, 2]
