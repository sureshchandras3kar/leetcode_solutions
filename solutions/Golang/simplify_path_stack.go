package main

import (
	"fmt"
	"strings"
)

// SimplifyPathStack simplifies an absolute path using a stack approach.
//
// Time: O(n) where n is the length of the path
// Space: O(n) for the stack storing directory names
func SimplifyPathStack(path string) string {
	// Split path by '/' and filter empty strings
	parts := strings.Split(path, "/")
	stack := []string{}

	for _, part := range parts {
		if part == "" || part == "." {
			// Skip empty strings and current directory references
			continue
		} else if part == ".." {
			// Go up one directory if possible
			if len(stack) > 0 {
				stack = stack[:len(stack)-1]
			}
		} else {
			// Add directory name to stack
			stack = append(stack, part)
		}
	}

	// Reconstruct the path
	return "/" + strings.Join(stack, "/")
}

func main() {
	fmt.Println(SimplifyPathStack("/a/./b/../../c/"))  // "/c"
	fmt.Println(SimplifyPathStack("/a/../../b/../c//.//"))  // "/c"
	fmt.Println(SimplifyPathStack("/a//b////c/d//././/.."))  // "/a/b/c"
	fmt.Println(SimplifyPathStack("/"))  // "/"
	fmt.Println(SimplifyPathStack("/../"))  // "/"
}
