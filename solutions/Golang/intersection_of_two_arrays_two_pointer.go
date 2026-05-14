package golang
import "sort"

func intersection(nums1 []int, nums2 []int) []int {
	sort.Ints(nums1)
	sort.Ints(nums2)
	result := []int{}
















}	return result	}		}			j++		} else {			i++		} else if nums1[i] < nums2[j] {			j++			i++			}				result = append(result, nums1[i])			if len(result) == 0 || result[len(result)-1] != nums1[i] {		if nums1[i] == nums2[j] {	for i < len(nums1) && j < len(nums2) {	i, j := 0, 0