package main

import (
	"fmt"
)

/**
 * Definition for a binary tree node.
 */
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

/**
 * Check if two binary trees are the same using BFS (level-order traversal).
 *
 * Time Complexity: O(min(m, n)) where m and n are the number of nodes
 * Space Complexity: O(min(w1, w2)) where w1 and w2 are the widths
 */
func IsSameTree(p *TreeNode, q *TreeNode) bool {
	// Use a queue to store pairs of nodes to compare
	queue := [][2]*TreeNode{{p, q}}

	for len(queue) > 0 {
		nodes := queue[0]
		queue = queue[1:]

		node1 := nodes[0]
		node2 := nodes[1]

		// Both are nil (equal)
		if node1 == nil && node2 == nil {
			continue
		}

		// One is nil or values differ (not equal)
		if node1 == nil || node2 == nil || node1.Val != node2.Val {
			return false
		}

		// Add children to queue for comparison
		queue = append(queue, [2]*TreeNode{node1.Left, node2.Left})
		queue = append(queue, [2]*TreeNode{node1.Right, node2.Right})
	}

	return true
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
