package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func AverageOfLevels(root *TreeNode) []float64 {
	sums := []int64{}
	counts := []int{}

	var dfs func(*TreeNode, int)
	dfs = func(node *TreeNode, level int) {
		if node == nil {
			return
		}

		if level == len(sums) {
			sums = append(sums, 0)
			counts = append(counts, 0)
		}

		sums[level] += int64(node.Val)
		counts[level]++

		dfs(node.Left, level+1)
		dfs(node.Right, level+1)
	}

	dfs(root, 0)

	result := make([]float64, len(sums))
	for i := 0; i < len(sums); i++ {
		result[i] = float64(sums[i]) / float64(counts[i])
	}

	return result
}

func main() {
	root := &TreeNode{Val: 3}
	root.Left = &TreeNode{Val: 9}
	root.Right = &TreeNode{Val: 20}
	root.Right.Left = &TreeNode{Val: 15}
	root.Right.Right = &TreeNode{Val: 7}

	result := AverageOfLevels(root)
	fmt.Println("Averages:", result)
}
