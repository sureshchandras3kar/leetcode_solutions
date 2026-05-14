package main

import "math"

type TreeNode struct { Val int; Left, Right *TreeNode }

func IsValidBST(root *TreeNode) bool {
    var dfs func(*TreeNode, float64, float64) bool
    dfs = func(n *TreeNode, minVal, maxVal float64) bool {
        if n == nil { return true }
        if float64(n.Val) <= minVal || float64(n.Val) >= maxVal { return false }
        return dfs(n.Left, minVal, float64(n.Val)) && dfs(n.Right, float64(n.Val), maxVal)
    }
    return dfs(root, math.Inf(-1), math.Inf(1))
}
