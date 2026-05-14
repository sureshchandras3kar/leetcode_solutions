class Node {
    constructor(val, isLeaf, topLeft = null, topRight = null, bottomLeft = null, bottomRight = null) {
        this.val = val;
        this.isLeaf = isLeaf;
        this.topLeft = topLeft;
        this.topRight = topRight;
        this.bottomLeft = bottomLeft;
        this.bottomRight = bottomRight;
    }
}

function construct(grid) {
    function dfs(top, left, size) {
        let allSame = true;
        const val = grid[top][left];
        
        for (let i = top; i < top + size && allSame; i++) {
            for (let j = left; j < left + size && allSame; j++) {
                if (grid[i][j] !== val) allSame = false;
            }
        }
        
        if (allSame) return new Node(val === 1, true);
        
        const half = Math.floor(size / 2);
        const node = new Node(true, false);
        node.topLeft = dfs(top, left, half);
        node.topRight = dfs(top, left + half, half);
        node.bottomLeft = dfs(top + half, left, half);
        node.bottomRight = dfs(top + half, left + half, half);
        return node;
    }
    
    return dfs(0, 0, grid.length);
}

const grid = [[1, 1], [1, 0]];
const root = construct(grid);
console.log(root.val, root.isLeaf);
