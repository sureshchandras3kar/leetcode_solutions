package main

import "fmt"

type Node struct {
	Val         bool
	IsLeaf      bool
	TopLeft     *Node
	TopRight    *Node
	BottomLeft  *Node
	BottomRight *Node
}

func construct(grid [][]int) *Node {
	return dfs(grid, 0, 0, len(grid))
}

func dfs(grid [][]int, top, left, size int) *Node {
	allSame := true
	val := grid[top][left]

	for i := top; i < top+size && allSame; i++ {
		for j := left; j < left+size && allSame; j++ {
			if grid[i][j] != val {
				allSame = false
			}
		}
	}

	if allSame {
		return &Node{Val: val == 1, IsLeaf: true}
	}

	half := size / 2
	node := &Node{Val: true, IsLeaf: false}
	node.TopLeft = dfs(grid, top, left, half)
	node.TopRight = dfs(grid, top, left+half, half)
	node.BottomLeft = dfs(grid, top+half, left, half)
	node.BottomRight = dfs(grid, top+half, left+half, half)
	return node
}

func main() {
	grid := [][]int{{1, 1}, {1, 0}}
	root := construct(grid)
	fmt.Println(root.Val, root.IsLeaf)
}
