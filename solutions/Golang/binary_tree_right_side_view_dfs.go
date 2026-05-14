package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func RightSideView(root *TreeNode) []int {
	result := []int{}

	var dfs func(*TreeNode, int)
	dfs = func(node *TreeNode, level int) {
		if node == nil {
			return
		}

		if level == len(result) {
			result = append(result, node.Val)
		}

		dfs(node.Right, level+1)
		dfs(node.Left, level+1)
	}

	dfs(root, 0)
	return result
}

func main() {
	root1 := &TreeNode{Val: 1}
	root1.Left = &TreeNode{Val: 2}
	root1.Right = &TreeNode{Val: 3}
	root1.Left.Right = &TreeNode{Val: 5}
	root1.Right.Right = &TreeNode{Val: 4}

	result1 := RightSideView(root1)
	fmt.Println("Right side view:", result1)
}
