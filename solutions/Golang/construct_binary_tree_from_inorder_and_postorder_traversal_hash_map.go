package main

import "fmt"

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

/*
buildTree constructs a binary tree from inorder and postorder traversal using HashMap.

Key insight:
- Postorder: left subtree, right subtree, root (last element is always root)
- Inorder: left subtree, root, right subtree

Use a HashMap to quickly find the root's position in inorder,
then recursively build left and right subtrees.

Time Complexity: O(n) where n is the number of nodes
Space Complexity: O(n) for HashMap and recursion call stack
*/
func buildTree(inorder []int, postorder []int) *TreeNode {
	if len(inorder) == 0 || len(postorder) == 0 {
		return nil
	}

	// Build map for O(1) inorder lookup
	inorderMap := make(map[int]int)
	for i, val := range inorder {
		inorderMap[val] = i
	}

	var build func(postStart, postEnd, inStart, inEnd int) *TreeNode
	build = func(postStart, postEnd, inStart, inEnd int) *TreeNode {
		/*
		Recursively build tree from postorder and inorder ranges.

		Args:
		    postStart, postEnd: Range in postorder array
		    inStart, inEnd: Range in inorder array
		*/

		if postStart > postEnd || inStart > inEnd {
			return nil
		}

		// Last element in postorder range is the root
		rootVal := postorder[postEnd]
		root := &TreeNode{Val: rootVal}

		// Find root position in inorder
		rootIdx := inorderMap[rootVal]

		// Number of nodes in left subtree
		leftSize := rootIdx - inStart

		// Recursively build left subtree
		root.Left = build(postStart, postStart+leftSize-1, inStart, rootIdx-1)

		// Recursively build right subtree
		root.Right = build(postStart+leftSize, postEnd-1, rootIdx+1, inEnd)

		return root
	}

	return build(0, len(postorder)-1, 0, len(inorder)-1)
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
