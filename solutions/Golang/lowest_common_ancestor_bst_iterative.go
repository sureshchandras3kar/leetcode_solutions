package main

import "fmt"

type TreeNode struct {
    Val   int
    Left  *TreeNode
    Right *TreeNode
}

func LowestCommonAncestorIterative(root *TreeNode, p *TreeNode, q *TreeNode) *TreeNode {
    current := root

    for current != nil {
        if current.Val > p.Val && current.Val > q.Val {
            // Both p and q are in left subtree
            current = current.Left
        } else if current.Val < p.Val && current.Val < q.Val {
            // Both p and q are in right subtree
            current = current.Right
        } else {
            // p and q are on different sides or one of them is current
            return current
        }
    }

    return root
}

func main() {
    root := &TreeNode{Val: 6}
    root.Left = &TreeNode{Val: 2}
    root.Right = &TreeNode{Val: 8}
    root.Left.Left = &TreeNode{Val: 0}
    root.Left.Right = &TreeNode{Val: 4}
    root.Left.Right.Left = &TreeNode{Val: 3}
    root.Left.Right.Right = &TreeNode{Val: 5}
    root.Right.Left = &TreeNode{Val: 7}
    root.Right.Right = &TreeNode{Val: 9}

    p := root.Left
    q := root.Left.Right
    fmt.Println(LowestCommonAncestorIterative(root, p, q).Val)  // 2
}
