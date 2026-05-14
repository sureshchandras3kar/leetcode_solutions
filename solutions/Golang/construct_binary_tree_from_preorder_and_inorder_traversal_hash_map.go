package main

/**
 * Definition for a binary tree node.
 */
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func buildTree(preorder []int, inorder []int) *TreeNode {
	if len(preorder) == 0 || len(inorder) == 0 {
		return nil
	}

	// Create a map for quick lookup of indices in inorder
	inorderMap := make(map[int]int)
	for i, val := range inorder {
		inorderMap[val] = i
	}

	var build func(int, int, int, int) *TreeNode
	build = func(preorderStart, preorderEnd, inorderStart, inorderEnd int) *TreeNode {
		if preorderStart > preorderEnd || inorderStart > inorderEnd {
			return nil
		}

		// Root is the first element in preorder
		rootVal := preorder[preorderStart]
		root := &TreeNode{Val: rootVal}

		// Find root position in inorder
		rootInorderIdx := inorderMap[rootVal]

		// Number of elements in left subtree
		leftSize := rootInorderIdx - inorderStart

		// Build left subtree
		root.Left = build(
			preorderStart+1,
			preorderStart+leftSize,
			inorderStart,
			rootInorderIdx-1,
		)

		// Build right subtree
		root.Right = build(
			preorderStart+leftSize+1,
			preorderEnd,
			rootInorderIdx+1,
			inorderEnd,
		)

		return root
	}

	return build(0, len(preorder)-1, 0, len(inorder)-1)
}

func main() {
	// Test 1: [3,9,20,15,7], [9,3,15,20,7]
	preorder1 := []int{3, 9, 20, 15, 7}
	inorder1 := []int{9, 3, 15, 20, 7}
	root1 := buildTree(preorder1, inorder1)
	println("Test 1 - Root:", root1.Val)  // 3

	// Test 2: [1], [1]
	preorder2 := []int{1}
	inorder2 := []int{1}
	root2 := buildTree(preorder2, inorder2)
	println("Test 2 - Root:", root2.Val)  // 1
}
