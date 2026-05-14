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
 * Count nodes in complete binary tree using binary search on node positions.
 * Uses existence check for node at each possible position.
 * Time: O(log² n), Space: O(log n) for recursion
 */
func countNodes(root *TreeNode) int {
    if root == nil {
        return 0
    }

    // Find height of tree
    height := 0
    node := root
    for node != nil {
        height++
        node = node.Left
    }

    // Binary search on number of nodes
    low := int64(1) << (height - 1)
    high := (int64(1) << height) - 1

    for low <= high {
        mid := (low + high + 1) / 2
        if exists(mid, height, root) {
            low = mid
        } else {
            high = mid - 1
        }
    }

    return int(low)
}

func exists(pos int64, height int, root *TreeNode) bool {
    /**Check if node at position pos exists.*/
    left := int64(0)
    right := (int64(1) << (height - 1)) - 1
    node := root

    for i := 0; i < height-1; i++ {
        mid := (left + right + 1) / 2
        if pos >= mid {
            node = node.Right
            left = mid
        } else {
            node = node.Left
            right = mid - 1
        }
    }

    return node != nil
}

func main() {
    fmt.Println("Example 1: Count complete tree nodes with binary search")
}
