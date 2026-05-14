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
 * Binary Search Tree Iterator using pre-computed slice.
 * Stores all in-order elements upfront.
 * Space: O(n), next() O(1), hasNext() O(1)
 */
type BSTIterator struct {
    arr   []int
    index int
}

func Constructor(root *TreeNode) BSTIterator {
    iterator := BSTIterator{arr: []int{}, index: 0}
    iterator.inorder(root)
    return iterator
}

func (this *BSTIterator) inorder(node *TreeNode) {
    /**Pre-compute in-order traversal into array.*/
    if node == nil {
        return
    }

    this.inorder(node.Left)
    this.arr = append(this.arr, node.Val)
    this.inorder(node.Right)
}

/**
 * Return next smallest element.
 * Time: O(1)
 */
func (this *BSTIterator) Next() int {
    val := this.arr[this.index]
    this.index++
    return val
}

/**
 * Check if there are more elements.
 * Time: O(1)
 */
func (this *BSTIterator) HasNext() bool {
    return this.index < len(this.arr)
}

func main() {
    fmt.Println("Example 1: BST Iterator using ArrayList")
}
