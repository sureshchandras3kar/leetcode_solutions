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
 * BFS Iterative approach to invert a binary tree.
 * Uses a queue to visit nodes level by level, swapping children.
 *
 * Time Complexity: O(n) - visit each node once
 * Space Complexity: O(w) - w = max width of tree (nodes at widest level)
 */
func invertTree(root *TreeNode) *TreeNode {
	if root == nil {
		return root
	}

	queue := make([]*TreeNode, 0)
	queue = append(queue, root)

	for len(queue) > 0 {
		node := queue[0]
		queue = queue[1:]

		// Swap left and right children
		node.Left, node.Right = node.Right, node.Left

		// Add children to queue for processing
		if node.Left != nil {
			queue = append(queue, node.Left)
		}
		if node.Right != nil {
			queue = append(queue, node.Right)
		}
	}

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
