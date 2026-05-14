/**
 * Simple Two Pointers Approach
 * Allow at most 2 occurrences by checking if current element differs from element 2 positions back.
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

    let k = 0;
    for (let i = 0; i < nums.length; i++) {
        // Always write first 2 elements, or if current differs from element 2 positions back
        if (k < 2 || nums[i] !== nums[k - 2]) {
            nums[k] = nums[i];
            k++;
        }
    }

    return k;
}

// Test cases
console.log(removeDuplicates([1, 1, 1, 2, 2, 3]));  // 5, nums = [1, 1, 2, 2, 3, _]
console.log(removeDuplicates([0, 0, 1, 1, 1, 1, 2, 3, 3]));  // 7, nums = [0, 0, 1, 1, 2, 3, 3, _, _]
console.log(removeDuplicates([1]));  // 1, nums = [1]
console.log(removeDuplicates([1, 2]));  // 2, nums = [1, 2]
