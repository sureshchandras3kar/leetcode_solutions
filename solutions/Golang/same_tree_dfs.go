package main

import "fmt"

/**
 * Definition for a binary tree node.
 */
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

/**
 * Check if two binary trees are the same using DFS (recursive).
 *
 * Time Complexity: O(min(m, n)) where m and n are the number of nodes
 * Space Complexity: O(min(h1, h2)) where h1 and h2 are the heights (call stack)
 */
func IsSameTree(p *TreeNode, q *TreeNode) bool {
	// Both nodes are nil (base case: equal)
	if p == nil && q == nil {
		return true
	}

	// One is nil, the other isn't (not equal)
	if p == nil || q == nil {
		return false
	}

	// Values differ (not equal)
	if p.Val != q.Val {
		return false
	}

	// Recursively check left and right subtrees
	return IsSameTree(p.Left, q.Left) && IsSameTree(p.Right, q.Right)
}

func main() {
	// Example 1: Same trees
	//     1           1
	//    / \         / \
	//   2   3       2   3
	p1 := &TreeNode{
		Val: 1,
		Left: &TreeNode{
			Val: 2,
		},
		Right: &TreeNode{
			Val: 3,
		},
	}

	q1 := &TreeNode{
		Val: 1,
		Left: &TreeNode{
			Val: 2,
		},
		Right: &TreeNode{
			Val: 3,
		},
	}

	fmt.Println(IsSameTree(p1, q1)) // Expected: true

	// Example 2: Different structure
	//     1           1
	//    /             \
	//   2               2
	p2 := &TreeNode{
		Val: 1,
		Left: &TreeNode{
			Val: 2,
		},
	}

	q2 := &TreeNode{
		Val: 1,
		Right: &TreeNode{
			Val: 2,
		},
	}

	fmt.Println(IsSameTree(p2, q2)) // Expected: false

	// Example 3: Different values
	//     1           1
	//    / \         / \
	//   2   1       1   2
	p3 := &TreeNode{
		Val: 1,
		Left: &TreeNode{
			Val: 2,
		},
		Right: &TreeNode{
			Val: 1,
		},
	}

	q3 := &TreeNode{
		Val: 1,
		Left: &TreeNode{
			Val: 1,
		},
		Right: &TreeNode{
			Val: 2,
		},
	}

	fmt.Println(IsSameTree(p3, q3)) // Expected: false

	// Example 4: Both empty
	fmt.Println(IsSameTree(nil, nil)) // Expected: true
}
