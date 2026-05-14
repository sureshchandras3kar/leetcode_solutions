package main

type TreeNode struct { Val int; Left, Right *TreeNode }

func ZigzagLevelOrder(root *TreeNode) [][]int {
    var result [][]int
    var dfs func(*TreeNode, int)
    dfs = func(n *TreeNode, level int) {
        if n == nil { return }
        if level == len(result) { result = append(result, []int{}) }
        if level%2 == 0 { result[level] = append(result[level], n.Val) }
        else { result[level] = append([]int{n.Val}, result[level]...) }
        dfs(n.Left, level + 1)
        dfs(n.Right, level + 1)
    }
    dfs(root, 0)
    return result
}
