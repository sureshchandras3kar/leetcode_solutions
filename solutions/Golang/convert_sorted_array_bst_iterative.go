package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func sortedArrayToBST(nums []int) *TreeNode {
	if len(nums) == 0 {
		return nil
	}

	root := &TreeNode{}
	type state struct {
		node  *TreeNode
		left  int
		right int
	}

	queue := []state{{root, 0, len(nums) - 1}}

	for len(queue) > 0 {
		curr := queue[0]
		queue = queue[1:]

		mid := curr.left + (curr.right-curr.left)/2
		curr.node.Val = nums[mid]

		if curr.left <= mid-1 {
			curr.node.Left = &TreeNode{}
			queue = append(queue, state{curr.node.Left, curr.left, mid - 1})
		}

		if mid+1 <= curr.right {
			curr.node.Right = &TreeNode{}
			queue = append(queue, state{curr.node.Right, mid + 1, curr.right})
		}
	}

	return root
}

func main() {
	nums := []int{-10, -3, 0, 5, 9}
	root := sortedArrayToBST(nums)
	fmt.Println(root.Val)  // 0
	fmt.Println(root.Left.Val)  // -3
	fmt.Println(root.Right.Val)  // 5
}
