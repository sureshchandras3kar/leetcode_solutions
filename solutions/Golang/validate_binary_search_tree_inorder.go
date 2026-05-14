package main

import "math"

type TreeNode struct { Val int; Left, Right *TreeNode }

func IsValidBST(root *TreeNode) bool {
    prev := []float64{math.Inf(-1)}
    var dfs func(*TreeNode) bool
    dfs = func(n *TreeNode) bool {
        if n == nil { return true }
        if !dfs(n.Left) { return false }
        if float64(n.Val) <= prev[0] { return false }
        prev[0] = float64(n.Val)
        return dfs(n.Right)
    }
    return dfs(root)
}
