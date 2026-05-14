package main

import "fmt"

type TreeNode struct {
    Val   int
    Left  *TreeNode
    Right *TreeNode
}

func kthSmallest_inorder(root *TreeNode, k int) int {
    var count = 0
    var result = -1

    var inorder func(*TreeNode)
    inorder = func(node *TreeNode) {
        if node == nil || result != -1 {
            return
        }

        inorder(node.Left)

        count++
        if count == k {
            result = node.Val
            return
        }

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

    fmt.Println(kthSmallest_inorder(root, 1))  // 1
    fmt.Println(kthSmallest_inorder(root, 3))  // 2
}
