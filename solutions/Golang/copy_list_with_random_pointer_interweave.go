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

    // First pass: create copies and interleave them
    // original -> copy -> original -> copy -> ...
    current := head
    for current != nil {
        copy := &Node{Val: current.Val}
        copy.Next = current.Next
        current.Next = copy
        current = copy.Next
    }

    // Second pass: set random pointers for copy nodes
    current = head
    for current != nil {
        if current.Random != nil {
            current.Next.Random = current.Random.Next
        }
        current = current.Next.Next
    }

    // Third pass: restore original list and extract copy list
    current = head
    copyHead := head.Next
    for current != nil {
        copy := current.Next
        current.Next = copy.Next
        if copy.Next != nil {
            copy.Next = copy.Next.Next
        }
        current = current.Next
    }

    return copyHead
}
