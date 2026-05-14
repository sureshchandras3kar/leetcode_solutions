package main

type TreeNode struct { Val int; Left, Right *TreeNode }

func LevelOrder(root *TreeNode) [][]int {
    var result [][]int
    if root == nil { return result }
    queue := []*TreeNode{root}
    for len(queue) > 0 {
        sz := len(queue)
        level := make([]int, 0)
        for i := 0; i < sz; i++ {
            n := queue[0]
            queue = queue[1:]
            level = append(level, n.Val)
            if n.Left != nil { queue = append(queue, n.Left) }
            if n.Right != nil { queue = append(queue, n.Right) }
        }
        result = append(result, level)
    }
    return result
}
