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

func GetMinimumDifferenceBFS(root *TreeNode) int {
    if root == nil {
        return 0
    }

    var stack []*TreeNode
    current := root
    var prev *int
    minDiff := math.MaxInt32

    for len(stack) > 0 || current != nil {
        // Go to leftmost node
        for current != nil {
            stack = append(stack, current)
            current = current.Left
        }

        // Pop from stack
        if len(stack) == 0 {
            break
        }
        current = stack[len(stack)-1]
        stack = stack[:len(stack)-1]

        // Process current node
        if prev != nil {
            diff := current.Val - *prev
            if diff < minDiff {
                minDiff = diff
            }
        }
        prev = &current.Val

        // Visit right subtree
        current = current.Right
    }

    return minDiff
}

func main() {
    root := &TreeNode{Val: 4}
    root.Left = &TreeNode{Val: 2}
    root.Right = &TreeNode{Val: 6}
    root.Left.Left = &TreeNode{Val: 1}
    root.Left.Right = &TreeNode{Val: 3}

    fmt.Println(GetMinimumDifferenceBFS(root))  // 1
}
