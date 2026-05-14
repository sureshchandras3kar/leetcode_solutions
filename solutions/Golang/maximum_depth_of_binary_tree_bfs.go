package main

/*
 * Find the maximum depth of a binary tree using BFS (level-order traversal).
 *
 * Time Complexity: O(n) where n is the number of nodes
 * Space Complexity: O(w) where w is the maximum width of the tree
 */

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func maxDepth(root *TreeNode) int {
	if root == nil {
		return 0
	}

	queue := []*TreeNode{root}
	depth := 0

	for len(queue) > 0 {
		depth++
		// Process all nodes at the current level
		levelSize := len(queue)
		nextQueue := []*TreeNode{}

		for i := 0; i < levelSize; i++ {
			node := queue[i]
			if node.Left != nil {
				nextQueue = append(nextQueue, node.Left)
			}
			if node.Right != nil {
				nextQueue = append(nextQueue, node.Right)
			}
		}

		queue = nextQueue
	}

	return depth
}

// Test cases
func main() {
	// Example 1: [3,9,20,null,null,15,7]
	//       3
	//      / \
	//     9  20
	//       /  \
	//      15   7
	root1 := &TreeNode{Val: 3}
	root1.Left = &TreeNode{Val: 9}
	root1.Right = &TreeNode{Val: 20}
	root1.Right.Left = &TreeNode{Val: 15}
	root1.Right.Right = &TreeNode{Val: 7}

	println(maxDepth(root1)) // Expected: 3

	// Example 2: [1,null,2]
	//     1
	//      \
	//       2
	root2 := &TreeNode{Val: 1}
	root2.Right = &TreeNode{Val: 2}

	println(maxDepth(root2)) // Expected: 2

	// Example 3: Empty tree
	var root3 *TreeNode = nil
	println(maxDepth(root3)) // Expected: 0
}
