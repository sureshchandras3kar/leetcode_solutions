class TreeNode {
    constructor(val = 0, left = null, right = null) {
        this.val = val;
        this.left = left;
        this.right = right;
    }
}

function sortedArrayToBST(nums) {
    if (nums.length === 0) return null;

    const root = new TreeNode();
    const queue = [[root, 0, nums.length - 1]];

    while (queue.length > 0) {
        const [node, left, right] = queue.shift();

        const mid = Math.floor((left + right) / 2);
        node.val = nums[mid];

        if (left <= mid - 1) {
            node.left = new TreeNode();
            queue.push([node.left, left, mid - 1]);
        }

        if (mid + 1 <= right) {
            node.right = new TreeNode();
            queue.push([node.right, mid + 1, right]);
        }
    }

    return root;
}

// Example usage
const nums = [-10, -3, 0, 5, 9];
const root = sortedArrayToBST(nums);
console.log(root.val);  // 0
console.log(root.left.val);  // -3
console.log(root.right.val);  // 5
