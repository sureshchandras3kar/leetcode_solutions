function rotateArrayExtraSpace(nums, k) {
    /**
     * Rotate array using extra space.
     *
     * Time: O(n) | Space: O(n)
     *
     * Approach: Create a new array where element at index i goes to index
     * (i + k) % n. Copy back to original array.
     */
    if (!nums || nums.length === 0 || k === 0) {
        return;
    }

    const n = nums.length;
    k = k % n;  // Handle k > n
    if (k === 0) {
        return;
    }

    // Create rotated result
    const rotated = new Array(n);
    for (let i = 0; i < n; i++) {
        rotated[(i + k) % n] = nums[i];
    }

    // Copy back to original array
    for (let i = 0; i < n; i++) {
        nums[i] = rotated[i];
    }
}

const nums = [1, 2, 3, 4, 5];
rotateArrayExtraSpace(nums, 2);
console.log(nums);  // [4, 5, 1, 2, 3]
