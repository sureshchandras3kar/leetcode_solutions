package main

import "fmt"

type TreeNode struct {
    Val   int
    Left  *TreeNode
    Right *TreeNode
}

func LowestCommonAncestorRecursive(root *TreeNode, p *TreeNode, q *TreeNode) *TreeNode {
    if root.Val > p.Val && root.Val > q.Val {
        // Both p and q are in left subtree
        return LowestCommonAncestorRecursive(root.Left, p, q)
    } else if root.Val < p.Val && root.Val < q.Val {
        // Both p and q are in right subtree
        return LowestCommonAncestorRecursive(root.Right, p, q)
    } else {
        // p and q are on different sides or one of them is root
        return root
    }
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
    fmt.Println(LowestCommonAncestorRecursive(root, p, q).Val)  // 2
}
