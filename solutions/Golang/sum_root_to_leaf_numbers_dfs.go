package main

import "fmt"

/**
 * Definition for a binary tree node.
 */
type TreeNode struct {
    Val   int
    Left  *TreeNode
    Right *TreeNode
}

/**
 * Sum all root-to-leaf numbers using DFS.
 * Build number by appending digits as we traverse down.
 * Time: O(n), Space: O(h) for recursion stack
 */
func sumNumbers(root *TreeNode) int {
    return dfs(root, 0)
}

func dfs(node *TreeNode, currentSum int) int {
    if node == nil {
        return 0
    }

    // Build number: multiply by 10 and add current digit
    currentSum = currentSum*10 + node.Val

    // Leaf node: return the complete number
    if node.Left == nil && node.Right == nil {
        return currentSum
    }

    // Recursively process children and sum
    return dfs(node.Left, currentSum) + dfs(node.Right, currentSum)
}

func main() {
    fmt.Println("Example 1: Sum root to leaf numbers with DFS")
}
