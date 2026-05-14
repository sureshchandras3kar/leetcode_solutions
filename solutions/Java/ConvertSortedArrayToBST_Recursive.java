class TreeNode {
    int val;
    TreeNode left;
    TreeNode right;
    TreeNode(int val) {
        this.val = val;
    }
}

class Solution {
    public TreeNode sortedArrayToBST(int[] nums) {
        return build(nums, 0, nums.length - 1);
    }

    private TreeNode build(int[] nums, int left, int right) {
        if (left > right) return null;

        int mid = left + (right - left) / 2;
        TreeNode root = new TreeNode(nums[mid]);
        root.left = build(nums, left, mid - 1);
        root.right = build(nums, mid + 1, right);
        return root;
    }
}

public class ConvertSortedArrayToBST_Recursive {
    public static void main(String[] args) {
        Solution sol = new Solution();
        int[] nums = {-10, -3, 0, 5, 9};
        TreeNode root = sol.sortedArrayToBST(nums);
        System.out.println(root.val);  // 0
        System.out.println(root.left.val);  // -3
        System.out.println(root.right.val);  // 5
    }
}
