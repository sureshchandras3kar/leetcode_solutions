package main

import (
	"fmt"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

/**
 * DFS Recursive approach to invert a binary tree.
 * Recursively swap left and right children for each node.
 *
 * Time Complexity: O(n) - visit each node once
 * Space Complexity: O(h) - recursion stack depth (h = height)
 */
func invertTree(root *TreeNode) *TreeNode {
	if root == nil {
		return nil
	}

	// Swap left and right children
	root.Left, root.Right = root.Right, root.Left

	// Recursively invert left and right subtrees
	invertTree(root.Left)
	invertTree(root.Right)

	return root
}

func main() {
	// Create tree:     1
	//                /   \
	//               2     3
	root := &TreeNode{
		Val: 1,
		Left: &TreeNode{
			Val: 2,
		},
		Right: &TreeNode{
			Val: 3,
		},
	}

	invertTree(root)

	// Expected:        1
	//                /   \
	//               3     2
	fmt.Printf("Root: %d\n", root.Val)  // 1
	fmt.Printf("Left: %d, Right: %d\n", root.Left.Val, root.Right.Val)  // 3, 2
}
