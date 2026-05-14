package main

import (
    "fmt"
    "math"
)

/**
 * Definition for a binary tree node.
 */
type TreeNode struct {
    Val   int
    Left  *TreeNode
    Right *TreeNode
}

/**
 * Find maximum path sum using DFS with max tracking.
 * Maintains maxSum as mutable reference updated during traversal.
 * Time: O(n), Space: O(h) for recursion stack
 */
func maxPathSum(root *TreeNode) int {
    maxSum := math.MinInt
    dfs(root, &maxSum)
    return maxSum
}

func dfs(node *TreeNode, maxSum *int) int64 {
    if node == nil {
        return 0
    }

    // Max single-path sum extending from this node
    leftSum := maxInt64(0, dfs(node.Left, maxSum))
    rightSum := maxInt64(0, dfs(node.Right, maxSum))

    // Path bending at this node
    pathSum := int64(node.Val) + leftSum + rightSum
    *maxSum = max(*maxSum, int(pathSum))

    // Return best single path extending downward
    return int64(node.Val) + maxInt64(leftSum, rightSum)
}

func maxInt64(a, b int64) int64 {
    if a > b {
        return a
    }
    return b
}

func max(a, b int) int {
    if a > b {
        return a
    }
    return b
}

func main() {
    fmt.Println("Example 1: Binary tree maximum path sum with max tracking")
}
