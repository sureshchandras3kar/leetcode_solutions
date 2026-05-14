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
 * Pair struct to hold node and sum
 */
type Pair struct {
    node      *TreeNode
    sum       int
}

/**
 * Check if tree has root-to-leaf path summing to targetSum using iterative BFS.
 * Queue stores (node, current_sum) pairs.
 * Time: O(n), Space: O(w) where w is max width
 */
func hasPathSum(root *TreeNode, targetSum int) bool {
    if root == nil {
        return false
    }

    queue := []Pair{{node: root, sum: root.Val}}

    for len(queue) > 0 {
        pair := queue[0]
        queue = queue[1:]

        node := pair.node
        currentSum := pair.sum

        // Check leaf node
        if node.Left == nil && node.Right == nil && currentSum == targetSum {
            return true
        }

        if node.Left != nil {
            queue = append(queue, Pair{node: node.Left, sum: currentSum + node.Left.Val})
        }
        if node.Right != nil {
            queue = append(queue, Pair{node: node.Right, sum: currentSum + node.Right.Val})
        }
    }

    return false
}

func main() {
    fmt.Println("Example 1: Path sum check with BFS")
}
