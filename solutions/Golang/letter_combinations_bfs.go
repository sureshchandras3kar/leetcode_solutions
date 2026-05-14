package main

import "fmt"

func letterCombinationsBFS(digits string) []string {
	if digits == "" {
		return []string{}
	}

	phoneMap := map[byte]string{
		'2': "abc",
		'3': "def",
		'4': "ghi",
		'5': "jkl",
		'6': "mno",
		'7': "pqrs",
		'8': "tuv",
		'9': "wxyz",
	}

	queue := []string{""}

	for i := 0; i < len(digits); i++ {
		digit := digits[i]
		letters := phoneMap[digit]

		var newQueue []string

		// Process all current combinations
		for _, current := range queue {
			// Create a new combination for each letter
			for _, letter := range letters {
				newQueue = append(newQueue, current+string(letter))
			}
		}

		queue = newQueue
	}

	return queue
}

func main() {
	fmt.Println(letterCombinationsBFS("23"))
	// Output: [ad ae af bd be bf cd ce cf]
}
