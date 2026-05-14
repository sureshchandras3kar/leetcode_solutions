class Node {
    constructor(val = 0, neighbors = []) {
        this.val = val;
        this.neighbors = neighbors;
    }
}

function cloneGraphDFS(node) {
    if (!node) return null;
    
    const visited = new Map();
    
    const dfs = (original) => {
        if (visited.has(original.val)) {
            return visited.get(original.val);
        }
        
        const cloned = new Node(original.val);
        visited.set(original.val, cloned);
        
        for (const neighbor of original.neighbors) {
            cloned.neighbors.push(dfs(neighbor));
        }
        
        return cloned;
    };
    
    return dfs(node);
}
