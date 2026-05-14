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
 * Check if tree has root-to-leaf path summing to targetSum using recursive DFS.
 * Time: O(n), Space: O(h) for recursion stack
 */
func hasPathSum(root *TreeNode, targetSum int) bool {
    if root == nil {
        return false
    }

    // Leaf node check
    if root.Left == nil && root.Right == nil {
        return root.Val == targetSum
    }

    // Subtract current value and check left and right subtrees
    remaining := targetSum - root.Val
    return hasPathSum(root.Left, remaining) || hasPathSum(root.Right, remaining)
}

func main() {
    fmt.Println("Example 1: Path sum check with DFS")
}
