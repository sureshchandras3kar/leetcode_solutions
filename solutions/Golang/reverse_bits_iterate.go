package main

import "fmt"

func reverseBitsIterate(num uint32) uint32 {
	result := uint32(0)
	for i := 0; i < 32; i++ {
		result = (result << 1) | (num & 1)
		num >>= 1
	}
	return result
}

func main() {
	fmt.Println(reverseBitsIterate(43261596))  // 964176192
}
