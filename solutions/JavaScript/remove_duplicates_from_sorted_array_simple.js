/**
 * Simple Pass Approach
 * Remove duplicates by iterating and comparing consecutive elements.
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

    let writeIdx = 0;
    for (let readIdx = 1; readIdx < nums.length; readIdx++) {
        if (nums[readIdx] !== nums[writeIdx]) {
            writeIdx++;
            nums[writeIdx] = nums[readIdx];
        }
    }

    return writeIdx + 1;
}

// Test cases
console.log(removeDuplicates([1, 1, 2]));  // 2, nums = [1, 2, _]
console.log(removeDuplicates([0, 0, 1, 1, 1, 2, 2, 3, 3, 4]));  // 5, nums = [0, 1, 2, 3, 4, _, _, _, _, _]
