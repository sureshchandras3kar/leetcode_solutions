package main

import (
	"fmt"
	"sort"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

// IsSymmetricBFS checks if a binary tree is symmetric using BFS with a queue.
//
// Time Complexity: O(n) - visit each node once
// Space Complexity: O(w) - queue width, where w is max nodes at any level
func isSymmetricBFS(root *TreeNode) bool {
	if root == nil {
		return true
	}

	queue := [][2]*TreeNode{
		{root.Left, root.Right},
	}

	for len(queue) > 0 {
		pair := queue[0]
		queue = queue[1:]

		left, right := pair[0], pair[1]

		// Both nodes are nil - continue (symmetric so far)
		if left == nil && right == nil {
			continue
		}

		// One node is nil or values differ - not symmetric
		if left == nil || right == nil {
			return false
		}
		if left.Val != right.Val {
			return false
		}

		// Add pairs for next level: left's left with right's right
		// and left's right with right's left (mirror pattern)
		queue = append(queue, [2]*TreeNode{left.Left, right.Right})
		queue = append(queue, [2]*TreeNode{left.Right, right.Left})
	}

	return true
}

func main() {
	// Example 1: Symmetric tree
	//       1
	//      / \
	//     2   2
	//    / \ / \
	//   3  4 4  3
	root1 := &TreeNode{Val: 1}
	root1.Left = &TreeNode{Val: 2}
	root1.Right = &TreeNode{Val: 2}
	root1.Left.Left = &TreeNode{Val: 3}
	root1.Left.Right = &TreeNode{Val: 4}
	root1.Right.Left = &TreeNode{Val: 4}
	root1.Right.Right = &TreeNode{Val: 3}
	fmt.Println(isSymmetricBFS(root1)) // true

	// Example 2: Not symmetric
	//       1
	//      / \
	//     2   2
	//      \   \
	//       3   3
	root2 := &TreeNode{Val: 1}
	root2.Left = &TreeNode{Val: 2}
	root2.Right = &TreeNode{Val: 2}
	root2.Left.Right = &TreeNode{Val: 3}
	root2.Right.Right = &TreeNode{Val: 3}
	fmt.Println(isSymmetricBFS(root2)) // false

	// Example 3: Single node
	root3 := &TreeNode{Val: 1}
	fmt.Println(isSymmetricBFS(root3)) // true
}
