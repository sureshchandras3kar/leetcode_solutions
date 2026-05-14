package main

type Node struct {
    Val       int
    Neighbors []*Node
}

func cloneGraphBFS(node *Node) *Node {
    if node == nil {
        return nil
    }
    
    visited := make(map[int]*Node)
    queue := []*Node{node}
    visited[node.Val] = &Node{Val: node.Val}
    
    for len(queue) > 0 {
        curr := queue[0]
        queue = queue[1:]
        
        for _, neighbor := range curr.Neighbors {
            if _, ok := visited[neighbor.Val]; !ok {
                visited[neighbor.Val] = &Node{Val: neighbor.Val}
                queue = append(queue, neighbor)
            }
            visited[curr.Val].Neighbors = append(visited[curr.Val].Neighbors, visited[neighbor.Val])
        }
    }
    
    return visited[node.Val]
}
