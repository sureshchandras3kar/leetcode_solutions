package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

/**
 * Find the lowest common ancestor using DFS recursion.
 *
 * Time Complexity: O(n) where n is the number of nodes
 * Space Complexity: O(h) where h is the height (call stack depth)
 */
func LowestCommonAncestor(root, p, q *TreeNode) *TreeNode {
	if root == nil {
		return nil
	}

	// If either p or q is the current node, it's a potential LCA
	if root == p || root == q {
		return root
	}

	// Search in left and right subtrees
	left := LowestCommonAncestor(root.Left, p, q)
	right := LowestCommonAncestor(root.Right, p, q)

	// If both p and q are found in different subtrees, root is LCA
	if left != nil && right != nil {
		return root
	}

	// If both are in one subtree, return that subtree's result
	if left != nil {
		return left
	}
	return right
}

func main() {
	// Example 1: [3,5,1,6,2,0,8,null,null,7,4]
	//       3
	//      / \
	//     5   1
	//    / \ / \
	//   6  2 0  8
	//     / \
	//    7   4
	root1 := &TreeNode{Val: 3}
	root1.Left = &TreeNode{Val: 5}
	root1.Right = &TreeNode{Val: 1}
	root1.Left.Left = &TreeNode{Val: 6}
	root1.Left.Right = &TreeNode{Val: 2}
	root1.Right.Left = &TreeNode{Val: 0}
	root1.Right.Right = &TreeNode{Val: 8}
	root1.Left.Right.Left = &TreeNode{Val: 7}
	root1.Left.Right.Right = &TreeNode{Val: 4}

	p1 := root1.Left  // Node 5
	q1 := root1.Right // Node 1
	result1 := LowestCommonAncestor(root1, p1, q1)
	fmt.Printf("LCA of %d and %d: %d\n", p1.Val, q1.Val, result1.Val) // Expected: 3

	// Example 2: Same tree, p=5, q=4
	p2 := root1.Left              // Node 5
	q2 := root1.Left.Right.Right  // Node 4
	result2 := LowestCommonAncestor(root1, p2, q2)
	fmt.Printf("LCA of %d and %d: %d\n", p2.Val, q2.Val, result2.Val) // Expected: 5

	// Example 3: [1,2]
	//     1
	//      \
	//       2
	root3 := &TreeNode{Val: 1}
	root3.Right = &TreeNode{Val: 2}

	p3 := root3       // Node 1
	q3 := root3.Right // Node 2
	result3 := LowestCommonAncestor(root3, p3, q3)
	fmt.Printf("LCA of %d and %d: %d\n", p3.Val, q3.Val, result3.Val) // Expected: 1
}
