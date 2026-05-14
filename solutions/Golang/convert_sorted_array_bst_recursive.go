package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func sortedArrayToBST(nums []int) *TreeNode {
	var build func(int, int) *TreeNode
	build = func(left, right int) *TreeNode {
		if left > right {
			return nil
		}

		mid := left + (right-left)/2
		root := &TreeNode{Val: nums[mid]}
		root.Left = build(left, mid-1)
		root.Right = build(mid+1, right)
		return root
	}

	return build(0, len(nums)-1)
}

func main() {
	nums := []int{-10, -3, 0, 5, 9}
	root := sortedArrayToBST(nums)
	fmt.Println(root.Val)  // 0
	fmt.Println(root.Left.Val)  // -3
	fmt.Println(root.Right.Val)  // 5
}
