class Node {
    constructor(val = 0, neighbors = []) {
        this.val = val;
        this.neighbors = neighbors;
    }
}

function cloneGraphBFS(node) {
    if (!node) return null;
    
    const visited = new Map();
    const queue = [node];
    visited.set(node.val, new Node(node.val));
    
    while (queue.length > 0) {
        const curr = queue.shift();
        
        for (const neighbor of curr.neighbors) {
            if (!visited.has(neighbor.val)) {
                visited.set(neighbor.val, new Node(neighbor.val));
                queue.push(neighbor);
            }
            visited.get(curr.val).neighbors.push(visited.get(neighbor.val));
        }
    }
    
    return visited.get(node.val);
}
