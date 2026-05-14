package main

import "fmt"

type TreeNode struct {
    Val   int
    Left  *TreeNode
    Right *TreeNode
}

func KthSmallestMorrisInorder(root *TreeNode, k int) int {
    var count int
    current := root

    for current != nil {
        if current.Left == nil {
            // Left is null, process current node
            count++
            if count == k {
                return current.Val
            }
            current = current.Right
        } else {
            // Find in-order predecessor
            predecessor := current.Left
            for predecessor.Right != nil && predecessor.Right != current {
                predecessor = predecessor.Right
            }

            if predecessor.Right == nil {
                // Create link to current node
                predecessor.Right = current
                current = current.Left
            } else {
                // Link exists, remove it and process current
                predecessor.Right = nil
                count++
                if count == k {
                    return current.Val
                }
                current = current.Right
            }
        }
    }

    return -1
}

func main() {
    root := &TreeNode{Val: 3}
    root.Left = &TreeNode{Val: 1}
    root.Right = &TreeNode{Val: 4}
    root.Left.Right = &TreeNode{Val: 2}

    fmt.Println(KthSmallestMorrisInorder(root, 1))  // 1
    fmt.Println(KthSmallestMorrisInorder(root, 3))  // 2
}
