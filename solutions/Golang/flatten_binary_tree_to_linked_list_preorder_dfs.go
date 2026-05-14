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
 * Flatten binary tree to linked list using pre-order DFS.
 * Recursively flattens left and right subtrees, then rewires pointers.
 * Time: O(n), Space: O(h) for recursion stack
 */
func flatten(root *TreeNode) {
    if root == nil {
        return
    }

    flatten(root.Left)
    flatten(root.Right)

    if root.Left != nil {
        // Find rightmost node in flattened left subtree
        rightmost := root.Left
        for rightmost.Right != nil {
            rightmost = rightmost.Right
        }

        // Attach right subtree to rightmost node
        rightmost.Right = root.Right
        // Move flattened left subtree to right
        root.Right = root.Left
        root.Left = nil
    }
}

func main() {
    fmt.Println("Example 1: Tree flattened to linked list using pre-order DFS")
}
