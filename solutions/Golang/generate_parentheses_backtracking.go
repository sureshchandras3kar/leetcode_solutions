package main

func generateParenthesis(n int) []string {
    result := []string{}
    backtrack(&result, "", 0, 0, n)
    return result
}

func backtrack(result *[]string, current string, left, right, n int) {
    if len(current) == 2*n {
        *result = append(*result, current)
        return
    }
    if left < n {
        backtrack(result, current+"(", left+1, right, n)
    }
    if right < left {
        backtrack(result, current+")", left, right+1, n)
    }
}
