package main

import "fmt"

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

/*
buildTree constructs a binary tree from inorder and postorder traversal using index tracking.

Key insight:
- Postorder: left subtree, right subtree, root (last element is always root)
- Inorder: left subtree, root, right subtree
- Use a pointer to track the current root in postorder (traverse from end to start)
- Find root in inorder to split into left and right subtrees

Time Complexity: O(n²) in worst case due to linear search for root in inorder
Space Complexity: O(h) for recursion call stack where h is height
*/
func buildTree(inorder []int, postorder []int) *TreeNode {
	if len(inorder) == 0 || len(postorder) == 0 {
		return nil
	}

	postIdx := len(postorder) - 1

	var build func(inStart, inEnd int) *TreeNode
	build = func(inStart, inEnd int) *TreeNode {
		/*
		Recursively build tree by processing postorder from right to left.

		Args:
		    inStart, inEnd: Range in inorder array
		*/

		if inStart > inEnd || postIdx < 0 {
			return nil
		}

		// Current postorder element (processing from end to start)
		rootVal := postorder[postIdx]
		postIdx--

		root := &TreeNode{Val: rootVal}

		// Find root position in inorder
		rootIdx := -1
		for i := inStart; i <= inEnd; i++ {
			if inorder[i] == rootVal {
				rootIdx = i
				break
			}
		}

		// Build right subtree first (postorder: left, right, root)
		// Since we traverse postorder backwards, right comes before left
		root.Right = build(rootIdx+1, inEnd)

		// Build left subtree
		root.Left = build(inStart, rootIdx-1)

		return root
	}

	return build(0, len(inorder)-1)
}

// Helper function for inorder traversal
func inorderTraversal(node *TreeNode) []int {
	if node == nil {
		return []int{}
	}
	result := inorderTraversal(node.Left)
	result = append(result, node.Val)
	result = append(result, inorderTraversal(node.Right)...)
	return result
}

// Helper function for postorder traversal
func postorderTraversal(node *TreeNode) []int {
	if node == nil {
		return []int{}
	}
	result := postorderTraversal(node.Left)
	result = append(result, postorderTraversal(node.Right)...)
	result = append(result, node.Val)
	return result
}

func main() {
	// Example 1: [3,9,20,null,null,15,7]
	//     3
	//    / \
	//   9  20
	//      / \
	//     15  7
	inorder1 := []int{9, 3, 15, 20, 7}
	postorder1 := []int{9, 15, 7, 20, 3}
	root1 := buildTree(inorder1, postorder1)

	fmt.Println("Inorder:", inorderTraversal(root1))     // Expected: [9 3 15 20 7]
	fmt.Println("Postorder:", postorderTraversal(root1)) // Expected: [9 15 7 20 3]

	// Example 2: Single node
	inorder2 := []int{1}
	postorder2 := []int{1}
	root2 := buildTree(inorder2, postorder2)
	fmt.Println("Single node:", inorderTraversal(root2))

	// Example 3: Left skewed tree
	inorder3 := []int{3, 2, 1}
	postorder3 := []int{3, 2, 1}
	root3 := buildTree(inorder3, postorder3)
	fmt.Println("Left skewed:", inorderTraversal(root3))
}
