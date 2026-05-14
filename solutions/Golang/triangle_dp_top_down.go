package main

import (
	"fmt"
)

func minimumTotalTopDown(triangle [][]int) int {
	/*
	DP Top-Down (Memoization) approach for Triangle problem.
	Starts from the top and explores paths downward.
	Time: O(n²), Space: O(n²) for memoization
	*/
	memo := make(map[string]int)

	var dfs func(int, int) int
	dfs = func(row, col int) int {
		// Base case: at the bottom
		if row == len(triangle)-1 {
			return triangle[row][col]
		}

		// Check memo
		key := fmt.Sprintf("%d,%d", row, col)
		if val, ok := memo[key]; ok {
			return val
		}

		// Current value + minimum of two possible next positions
		next1 := dfs(row+1, col)
		next2 := dfs(row+1, col+1)
		result := triangle[row][col]
		if next1 < next2 {
			result += next1
		} else {
			result += next2
		}
		memo[key] = result
		return result
	}

	return dfs(0, 0)
}

func main() {
	triangle := [][]int{
		{2},
		{3, 4},
		{6, 5, 7},
		{4, 1, 8, 3},
	}
	fmt.Println(minimumTotalTopDown(triangle))  // 11
}
