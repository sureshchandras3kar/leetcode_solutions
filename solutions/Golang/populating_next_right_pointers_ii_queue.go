package main

import "fmt"

/**
 * Definition for a Node.
 */
type Node struct {
    Val   int
    Left  *Node
    Right *Node
    Next  *Node
}

/**
 * Populates next pointers using level-order BFS queue.
 * Time: O(n), Space: O(w) where w is max width
 */
func connect(root *Node) *Node {
    if root == nil {
        return root
    }

    queue := []*Node{root}

    for len(queue) > 0 {
        levelSize := len(queue)
        var prev *Node

        for i := 0; i < levelSize; i++ {
            node := queue[0]
            queue = queue[1:]

            if prev != nil {
                prev.Next = node
            }
            prev = node

            if node.Left != nil {
                queue = append(queue, node.Left)
            }
            if node.Right != nil {
                queue = append(queue, node.Right)
            }
        }
    }

    return root
}

func main() {
    fmt.Println("Example 1: Tree with next pointers connected via queue")
}
