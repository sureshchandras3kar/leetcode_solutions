package main

func topKFrequentBucket(nums []int, k int) []int {
	freq := map[int]int{}
	for _, num := range nums {
		freq[num]++
	}

	n := len(nums)
	buckets := make([][]int, n+1)
	for num, count := range freq {
		buckets[count] = append(buckets[count], num)
	}

	result := make([]int, 0, k)
	for i := n; i >= 1 && len(result) < k; i-- {
		for _, num := range buckets[i] {
			result = append(result, num)
			if len(result) == k {
				return result
			}
		}
	}
	return result
}
