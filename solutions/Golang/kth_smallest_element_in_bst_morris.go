package main

import "fmt"

type TreeNode struct {
    Val   int
    Left  *TreeNode
    Right *TreeNode
}

func kthSmallest_morris(root *TreeNode, k int) int {
    count := 0
    current := root

    for current != nil {
        if current.Left == nil {
            count++
            if count == k {
                return current.Val
            }
            current = current.Right
        } else {
            predecessor := current.Left
            for predecessor.Right != nil && predecessor.Right != current {
                predecessor = predecessor.Right
            }

            if predecessor.Right == nil {
                predecessor.Right = current
                current = current.Left
            } else {
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

    fmt.Println(kthSmallest_morris(root, 1))  // 1
    fmt.Println(kthSmallest_morris(root, 3))  // 2
}
