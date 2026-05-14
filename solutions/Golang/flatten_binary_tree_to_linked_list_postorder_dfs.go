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
 * Flatten binary tree to linked list using post-order DFS.
 * Uses previous pointer to track last visited node in reverse in-order.
 * Time: O(n), Space: O(h) for recursion stack
 */
func flatten(root *TreeNode) {
    var prev *TreeNode
    dfs(root, &prev)
}

func dfs(node *TreeNode, prev **TreeNode) {
    if node == nil {
        return
    }

    // Post-order: right, left, then process node
    dfs(node.Right, prev)
    dfs(node.Left, prev)

    node.Right = *prev
    node.Left = nil
    *prev = node
}

func main() {
    fmt.Println("Example 1: Tree flattened to linked list using post-order DFS")
}
