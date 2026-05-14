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
 * Populates next pointers using pre-computed links without queue.
 * Uses next pointers of parent level to traverse current level.
 * Time: O(n), Space: O(1)
 */
func connect(root *Node) *Node {
    if root == nil {
        return root
    }

    leftmost := root

    for leftmost != nil {
        var prev *Node
        current := leftmost

        for current != nil {
            if current.Left != nil {
                if prev != nil {
                    prev.Next = current.Left
                }
                prev = current.Left
            }

            if current.Right != nil {
                if prev != nil {
                    prev.Next = current.Right
                }
                prev = current.Right
            }

            current = current.Next
        }

        leftmost = leftmost.Left
    }

    return root
}

func main() {
    fmt.Println("Example 1: Tree with next pointers connected via pre-computed links")
}
