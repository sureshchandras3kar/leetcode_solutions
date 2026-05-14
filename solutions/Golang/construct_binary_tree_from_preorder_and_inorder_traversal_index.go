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

	preorderIdx := 0

	var build func(int, int) *TreeNode
	build = func(inorderStart, inorderEnd int) *TreeNode {
		if inorderStart > inorderEnd || preorderIdx >= len(preorder) {
			return nil
		}

		// Root is the current element in preorder
		rootVal := preorder[preorderIdx]
		preorderIdx++
		root := &TreeNode{Val: rootVal}

		// Find root position in inorder
		rootInorderIdx := -1
		for i := inorderStart; i <= inorderEnd; i++ {
			if inorder[i] == rootVal {
				rootInorderIdx = i
				break
			}
		}

		// Build left subtree
		root.Left = build(inorderStart, rootInorderIdx-1)

		// Build right subtree
		root.Right = build(rootInorderIdx+1, inorderEnd)

		return root
	}

	return build(0, len(inorder)-1)
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
