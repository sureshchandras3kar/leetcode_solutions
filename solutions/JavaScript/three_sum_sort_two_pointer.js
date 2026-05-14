function threeSumSortTwoPointer(nums) {
    nums.sort((a, b) => a - b);
    const result = [];
    const n = nums.length;

    for (let i = 0; i < n - 2; i++) {
        if (nums[i] > 0) break;
        if (i > 0 && nums[i] === nums[i - 1]) continue;
        let left = i + 1, right = n - 1;
        while (left < right) {
            const s = nums[i] + nums[left] + nums[right];
            if (s === 0) {
                result.push([nums[i], nums[left], nums[right]]);
                while (left < right && nums[left] === nums[left + 1]) left++;
                while (left < right && nums[right] === nums[right - 1]) right--;
                left++;
                right--;
            } else if (s < 0) {
                left++;
            } else {
                right--;
            }
        }
    }

    return result;
}

console.log(threeSumSortTwoPointer([-1, 0, 1, 2, -1, -4])); // [[-1,-1,2],[-1,0,1]]
console.log(threeSumSortTwoPointer([0, 1, 1]));               // []
console.log(threeSumSortTwoPointer([0, 0, 0]));               // [[0,0,0]]
