package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

// IsSymmetricDFS checks if a binary tree is symmetric using DFS recursion.
//
// Time Complexity: O(n) - visit each node once
// Space Complexity: O(h) - recursion stack, where h is height
func isSymmetricDFS(root *TreeNode) bool {
	return isMirror(root, root)
}

// isMirror checks if two subtrees are mirrors of each other.
func isMirror(left, right *TreeNode) bool {
	// Both nodes are nil - symmetric
	if left == nil && right == nil {
		return true
	}

	// One node is nil or values differ - not symmetric
	if left == nil || right == nil {
		return false
	}
	if left.Val != right.Val {
		return false
	}

	// Recursively check: left's left with right's right
	// and left's right with right's left (mirror pattern)
	return isMirror(left.Left, right.Right) &&
		isMirror(left.Right, right.Left)
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
	fmt.Println(isSymmetricDFS(root1)) // true

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
	fmt.Println(isSymmetricDFS(root2)) // false
}
