function rotateArrayReverse(nums, k) {
    /**
     * Rotate array in-place using reverse trick.
     *
     * Time: O(n) | Space: O(1)
     *
     * Approach: Reverse the entire array, then reverse first k elements,
     * then reverse remaining n-k elements. This achieves rotation without
     * extra space.
     */
    if (!nums || nums.length === 0 || k === 0) {
        return;
    }

    const n = nums.length;
    k = k % n;  // Handle k > n
    if (k === 0) {
        return;
    }

    const reverse = (start, end) => {
        while (start < end) {
            [nums[start], nums[end]] = [nums[end], nums[start]];
            start++;
            end--;
        }
    };

    // Reverse entire array: [1,2,3,4,5] -> [5,4,3,2,1]
    reverse(0, n - 1);
    // Reverse first k: [5,4,3,2,1] -> [3,4,5,2,1]
    reverse(0, k - 1);
    // Reverse rest: [3,4,5,2,1] -> [3,4,5,1,2]
    reverse(k, n - 1);
}

const nums = [1, 2, 3, 4, 5];
rotateArrayReverse(nums, 2);
console.log(nums);  // [4, 5, 1, 2, 3]
