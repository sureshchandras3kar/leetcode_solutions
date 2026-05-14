package main

type TreeNode struct { Val int; Left, Right *TreeNode }

func ZigzagLevelOrder(root *TreeNode) [][]int {
    var result [][]int; if root == nil { return result }
    queue := []*TreeNode{root}; level := 0
    for len(queue) > 0 {
        sz := len(queue); var levelVals []int
        for i := 0; i < sz; i++ {
            n := queue[0]; queue = queue[1:]
            if level%2 == 0 { levelVals = append(levelVals, n.Val) }
            else { levelVals = append([]int{n.Val}, levelVals...) }
            if n.Left != nil { queue = append(queue, n.Left) }
            if n.Right != nil { queue = append(queue, n.Right) }
        }
        result = append(result, levelVals)
        level++
    }
    return result
}
