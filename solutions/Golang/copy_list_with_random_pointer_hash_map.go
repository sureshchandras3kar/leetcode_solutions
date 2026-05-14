package golang

type Node struct {
    Val    int
    Next   *Node
    Random *Node
}

func copyRandomList(head *Node) *Node {
    if head == nil {
        return nil
    }

    // Map original nodes to their copies
    nodeMap := make(map[*Node]*Node)

    // First pass: create all copy nodes
    current := head
    for current != nil {
        nodeMap[current] = &Node{Val: current.Val}
        current = current.Next
    }

    // Second pass: set next and random pointers
    current = head
    for current != nil {
        copyNode := nodeMap[current]
        if current.Next != nil {
            copyNode.Next = nodeMap[current.Next]
        }
        if current.Random != nil {
            copyNode.Random = nodeMap[current.Random]
        }
        current = current.Next
    }

    return nodeMap[head]
}
