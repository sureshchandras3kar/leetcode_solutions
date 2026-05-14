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

func getMinimumDifference_inorder(root *TreeNode) int {
    minDiff := math.MaxInt32
    var prev *int

    var inorder func(*TreeNode)
    inorder = func(node *TreeNode) {
        if node == nil {
            return
        }

        inorder(node.Left)

        if prev != nil {
            minDiff = min(minDiff, node.Val-*prev)
        }
        prev = &node.Val

        inorder(node.Right)
    }

    inorder(root)
    return minDiff
}

func min(a, b int) int {
    if a < b {
        return a
    }
    return b
}

func main() {
    root := &TreeNode{Val: 4}
    root.Left = &TreeNode{Val: 2}
    root.Right = &TreeNode{Val: 6}
    root.Left.Left = &TreeNode{Val: 1}
    root.Left.Right = &TreeNode{Val: 3}

    fmt.Println(getMinimumDifference_inorder(root))  // 1
}
