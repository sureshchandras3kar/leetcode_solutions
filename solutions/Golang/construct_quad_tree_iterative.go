package main

import (
	"fmt"
	"container/list"
)

type Node struct {
	Val         bool
	IsLeaf      bool
	TopLeft     *Node
	TopRight    *Node
	BottomLeft  *Node
	BottomRight *Node
}

func construct(grid [][]int) *Node {
	n := len(grid)
	type Task struct {
		top, left, size int
	}

	queue := list.New()
	queue.PushBack(Task{0, 0, n})
	var root *Node

	for queue.Len() > 0 {
		elem := queue.Front()
		queue.Remove(elem)
		task := elem.Value.(Task)

		allSame := true
		val := grid[task.top][task.left]
		for i := task.top; i < task.top+task.size && allSame; i++ {
			for j := task.left; j < task.left+task.size && allSame; j++ {
				if grid[i][j] != val {
					allSame = false
				}
			}
		}

		node := &Node{Val: val == 1, IsLeaf: allSame}
		if root == nil {
			root = node
		}

		if !allSame {
			half := task.size / 2
			queue.PushBack(Task{task.top, task.left, half})
			queue.PushBack(Task{task.top, task.left + half, half})
			queue.PushBack(Task{task.top + half, task.left, half})
			queue.PushBack(Task{task.top + half, task.left + half, half})
		}
	}

	return root
}

func main() {
	grid := [][]int{{1, 1}, {1, 0}}
	root := construct(grid)
	fmt.Println(root.Val, root.IsLeaf)
}
