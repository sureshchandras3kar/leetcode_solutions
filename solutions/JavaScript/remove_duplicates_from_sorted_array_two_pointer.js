/**
 * Two-Pointer In-Place Approach
 * Remove duplicates from sorted array in-place and return the length of the new array.
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

    let k = 1;  // First element is always unique
    for (let i = 1; i < nums.length; i++) {
        if (nums[i] !== nums[i - 1]) {
            nums[k] = nums[i];
            k++;
        }
    }

    return k;
}

// Test cases
console.log(removeDuplicates([1, 1, 2]));  // 2, nums = [1, 2, _]
console.log(removeDuplicates([0, 0, 1, 1, 1, 2, 2, 3, 3, 4]));  // 5, nums = [0, 1, 2, 3, 4, _, _, _, _, _]
