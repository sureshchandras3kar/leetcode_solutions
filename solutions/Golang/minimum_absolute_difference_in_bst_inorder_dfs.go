package main

import (
    "fmt"
    "math"
)

type TreeNode struct {
    Val   int
    Left  *TreeNode
    Right *TreeNode
}

func GetMinimumDifferenceInorderDFS(root *TreeNode) int {
    minDiff := math.MaxInt32
    var prev *int

    var inorder func(*TreeNode)
    inorder = func(node *TreeNode) {
        if node == nil {
            return
        }

        // Traverse left subtree
        inorder(node.Left)

        // Process current node
        if prev != nil {
            diff := node.Val - *prev
            if diff < minDiff {
                minDiff = diff
            }
        }
        prev = &node.Val

        // Traverse right subtree
        inorder(node.Right)
    }

    inorder(root)
    return minDiff
}

func main() {
    root := &TreeNode{Val: 4}
    root.Left = &TreeNode{Val: 2}
    root.Right = &TreeNode{Val: 6}
    root.Left.Left = &TreeNode{Val: 1}
    root.Left.Right = &TreeNode{Val: 3}

    fmt.Println(GetMinimumDifferenceInorderDFS(root))  // 1
}
