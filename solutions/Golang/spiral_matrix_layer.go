package main

import "fmt"

func spiralMatrixLayer(matrix [][]int) []int {
	if len(matrix) == 0 || len(matrix[0]) == 0 {
		return []int{}
	}

	result := []int{}
	m, n := len(matrix), len(matrix[0])
	layers := (min(m, n) + 1) / 2

	for layer := 0; layer < layers; layer++ {
		top := layer
		bottom := m - 1 - layer
		left := layer
		right := n - 1 - layer

		// Traverse right
		for col := left; col <= right; col++ {
			result = append(result, matrix[top][col])
		}

		// Traverse down
		for row := top + 1; row <= bottom; row++ {
			result = append(result, matrix[row][right])
		}

		// Traverse left (if there's a row remaining)
		if top < bottom {
			for col := right - 1; col >= left; col-- {
				result = append(result, matrix[bottom][col])
			}
		}

		// Traverse up (if there's a column remaining)
		if left < right {
			for row := bottom - 1; row > top; row-- {
				result = append(result, matrix[row][left])
			}
		}
	}

	return result
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func main() {
	matrix := [][]int{{1, 2, 3}, {4, 5, 6}, {7, 8, 9}}
	result := spiralMatrixLayer(matrix)
	fmt.Println(result)
}
