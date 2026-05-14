package main

import "fmt"

func isPalindrome(x int) bool {
    if x < 0 {
        return false
    }
    if x == 0 {
        return true
    }
    if x%10 == 0 {
        return false
    }
    numReversed := 0
    originalX := x
    for originalX > 0 {
        lastDigit := originalX % 10
        numReversed = numReversed*10 + lastDigit
        originalX /= 10
    }
    return x == numReversed
}

func main() {
    fmt.Println(isPalindrome(121))  // true
    fmt.Println(isPalindrome(-121)) // false
    fmt.Println(isPalindrome(10))   // false
}
