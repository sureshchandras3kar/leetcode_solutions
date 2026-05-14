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
 * Binary Search Tree Iterator using stack for in-order traversal.
 * Implements lazy evaluation: next() O(1) amortized, hasNext() O(1).
 * Space: O(h) where h is height
 */
type BSTIterator struct {
    stack []*TreeNode
}

func Constructor(root *TreeNode) BSTIterator {
    iterator := BSTIterator{stack: []*TreeNode{}}
    iterator.pushLeft(root)
    return iterator
}

func (this *BSTIterator) pushLeft(node *TreeNode) {
    /**Push all left nodes onto stack.*/
    for node != nil {
        this.stack = append(this.stack, node)
        node = node.Left
    }
}

/**
 * Return next smallest element.
 * Time: O(1) amortized
 */
func (this *BSTIterator) Next() int {
    node := this.stack[len(this.stack)-1]
    this.stack = this.stack[:len(this.stack)-1]

    if node.Right != nil {
        this.pushLeft(node.Right)
    }

    return node.Val
}

/**
 * Check if there are more elements.
 * Time: O(1)
 */
func (this *BSTIterator) HasNext() bool {
    return len(this.stack) > 0
}

func main() {
    fmt.Println("Example 1: BST Iterator using stack")
}
