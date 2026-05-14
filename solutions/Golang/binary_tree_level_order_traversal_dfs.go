package main

type TreeNode struct { Val int; Left, Right *TreeNode }

func LevelOrder(root *TreeNode) [][]int {
    var result [][]int
    var dfs func(*TreeNode, int)
    dfs = func(n *TreeNode, level int) {
        if n == nil { return }
        if level == len(result) { result = append(result, []int{}) }
        result[level] = append(result[level], n.Val)
        dfs(n.Left, level + 1)
        dfs(n.Right, level + 1)
    }
    dfs(root, 0)
    return result
}
