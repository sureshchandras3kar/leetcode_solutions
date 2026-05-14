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
 * Sum all root-to-leaf numbers using iterative BFS.
 * Queue stores (node, current_number) pairs.
 * Time: O(n), Space: O(w) where w is max width
 */
func sumNumbers(root *TreeNode) int {
    if root == nil {
        return 0
    }

    queue := []Pair{{node: root, sum: root.Val}}
    total := 0

    for len(queue) > 0 {
        pair := queue[0]
        queue = queue[1:]

        node := pair.node
        currentSum := pair.sum

        // Leaf node: add to total
        if node.Left == nil && node.Right == nil {
            total += currentSum
            continue
        }

        if node.Left != nil {
            queue = append(queue, Pair{node: node.Left, sum: currentSum*10 + node.Left.Val})
        }
        if node.Right != nil {
            queue = append(queue, Pair{node: node.Right, sum: currentSum*10 + node.Right.Val})
        }
    }

    return total
}

func main() {
    fmt.Println("Example 1: Sum root to leaf numbers with BFS")
}
