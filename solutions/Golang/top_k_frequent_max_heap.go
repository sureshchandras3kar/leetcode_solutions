package main

import "sort"

func topKFrequent(nums []int, k int) []int {
	freq := map[int]int{}
	for _, num := range nums {
		freq[num]++
	}

	type pair struct {
		num, count int
	}
	pairs := make([]pair, 0, len(freq))
	for num, count := range freq {
		pairs = append(pairs, pair{num, count})
	}
	// Sort by frequency descending
	sort.Slice(pairs, func(i, j int) bool {
		return pairs[i].count > pairs[j].count
	})

	result := make([]int, k)
	for i := 0; i < k; i++ {
		result[i] = pairs[i].num
	}
	return result
}
