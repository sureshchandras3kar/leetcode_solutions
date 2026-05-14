class TreeNode {
    constructor(val = 0, left = null, right = null) {
        this.val = val;
        this.left = left;
        this.right = right;
    }
}

function sortedArrayToBST(nums) {
    function build(left, right) {
        if (left > right) return null;

        const mid = Math.floor((left + right) / 2);
        const root = new TreeNode(nums[mid]);
        root.left = build(left, mid - 1);
        root.right = build(mid + 1, right);
        return root;
    }

    return build(0, nums.length - 1);
}

// Example usage
const nums = [-10, -3, 0, 5, 9];
const root = sortedArrayToBST(nums);
console.log(root.val);  // 0
console.log(root.left.val);  // -3
console.log(root.right.val);  // 5
