package main

import (
	"fmt"
	"strings"
)

// SimplifyPathParsing simplifies an absolute path using string parsing.
//
// Time: O(n) where n is the length of the path
// Space: O(n) for the result string
func SimplifyPathParsing(path string) string {
	canonical := "/"
	i := 0

	for i < len(path) {
		// Skip slashes
		for i < len(path) && path[i] == '/' {
			i++
		}

		if i >= len(path) {
			break
		}

		// Extract the next component
		j := i
		for j < len(path) && path[j] != '/' {
			j++
		}

		component := path[i:j]

		if component == ".." {
			// Go up one level
			lastSlash := strings.LastIndex(canonical[:len(canonical)-1], "/")
			if lastSlash >= 0 {  // Don't go above root
				canonical = canonical[:lastSlash+1]
			} else if canonical != "/" {
				canonical = "/"
			}
		} else if component != "." {
			// Add the component to canonical path
			if canonical != "/" {
				canonical += "/"
			}
			canonical += component
		}

		i = j
	}

	return canonical
}

func main() {
	fmt.Println(SimplifyPathParsing("/a/./b/../../c/"))  // "/c"
	fmt.Println(SimplifyPathParsing("/a/../../b/../c//.//"))  // "/c"
	fmt.Println(SimplifyPathParsing("/a//b////c/d//././/.."))  // "/a/b/c"
	fmt.Println(SimplifyPathParsing("/"))  // "/"
	fmt.Println(SimplifyPathParsing("/../"))  // "/"
}
