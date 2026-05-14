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
 * Find maximum path sum in binary tree using post-order DFS.
 * A path can pass through any node (not just root to leaf).
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

    // Max gain from left and right subtrees (at least 0 if negative)
    leftGain := max(0, dfs(node.Left, maxSum))
    rightGain := max(0, dfs(node.Right, maxSum))

    // Max path through this node (may bend at this node)
    maxPathThroughNode := int64(node.Val) + leftGain + rightGain
    *maxSum = max(*maxSum, int(maxPathThroughNode))

    // Return max path extending downward from this node
    return int64(node.Val) + max(leftGain, rightGain)
}

func max(a, b int64) int64 {
    if a > b {
        return a
    }
    return b
}

func main() {
    fmt.Println("Example 1: Binary tree maximum path sum with DFS")
}
