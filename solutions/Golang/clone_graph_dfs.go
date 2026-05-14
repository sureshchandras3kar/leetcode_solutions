package main

type Node struct {
    Val       int
    Neighbors []*Node
}

func cloneGraphDFS(node *Node) *Node {
    if node == nil {
        return nil
    }
    
    visited := make(map[int]*Node)
    return dfsDFS(node, visited)
}

func dfsDFS(node *Node, visited map[int]*Node) *Node {
    if cloned, ok := visited[node.Val]; ok {
        return cloned
    }
    
    cloned := &Node{Val: node.Val}
    visited[node.Val] = cloned
    
    for _, neighbor := range node.Neighbors {
        cloned.Neighbors = append(cloned.Neighbors, dfsDFS(neighbor, visited))
    }
    
    return cloned
}
