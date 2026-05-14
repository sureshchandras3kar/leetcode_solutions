package main

import "fmt"

type TreeNode struct {
    Val   int
    Left  *TreeNode
    Right *TreeNode
}

func KthSmallestInorderDFS(root *TreeNode, k int) int {
    var count int
    var result int = -1

    var inorder func(*TreeNode)
    inorder = func(node *TreeNode) {
        if node == nil || result != -1 {
            return
        }

        // Traverse left subtree
        inorder(node.Left)

        // Process current node
        count++
        if count == k {
            result = node.Val
            return
        }

        // Traverse right subtree
        inorder(node.Right)
    }

    inorder(root)
    return result
}

func main() {
    root := &TreeNode{Val: 3}
    root.Left = &TreeNode{Val: 1}
    root.Right = &TreeNode{Val: 4}
    root.Left.Right = &TreeNode{Val: 2}

    fmt.Println(KthSmallestInorderDFS(root, 1))  // 1
    fmt.Println(KthSmallestInorderDFS(root, 3))  // 2
}
