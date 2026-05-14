package main

import "fmt"

func numberOf1BitsLoop(num uint32) int {
	count := 0
	for num > 0 {
		count += int(num & 1)
		num >>= 1
	}
	return count
}

func main() {
	fmt.Println(numberOf1BitsLoop(11))  // 3
	fmt.Println(numberOf1BitsLoop(128))  // 1
}
