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
 * Count nodes in complete binary tree using level calculation.
 * For complete tree, if left height == right height, left is perfect.
 * Time: O(log² n), Space: O(log n) for recursion
 */
func countNodes(root *TreeNode) int {
    if root == nil {
        return 0
    }

    // Calculate left and right heights
    leftHeight := 0
    leftNode := root.Left
    for leftNode != nil {
        leftHeight++
        leftNode = leftNode.Left
    }

    rightHeight := 0
    rightNode := root.Right
    for rightNode != nil {
        rightHeight++
        rightNode = rightNode.Right
    }

    if leftHeight == rightHeight {
        // Left subtree is perfect: 2^(h+1) - 1 nodes + root + recursively count right
        return (1 << (leftHeight + 1)) - 1 + countNodes(root.Right)
    } else {
        // Right subtree is perfect: 2^h - 1 nodes + root + recursively count left
        return (1 << (rightHeight + 1)) - 1 + countNodes(root.Left)
    }
}

func main() {
    fmt.Println("Example 1: Count complete tree nodes with level calculation")
}
