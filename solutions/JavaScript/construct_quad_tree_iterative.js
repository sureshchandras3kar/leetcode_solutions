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
    const n = grid.length;
    const queue = [[0, 0, n]];
    let root = null;
    
    while (queue.length > 0) {
        const [top, left, size] = queue.shift();
        
        let allSame = true;
        const val = grid[top][left];
        for (let i = top; i < top + size && allSame; i++) {
            for (let j = left; j < left + size && allSame; j++) {
                if (grid[i][j] !== val) allSame = false;
            }
        }
        
        const node = new Node(val === 1, allSame);
        if (root === null) root = node;
        
        if (!allSame) {
            const half = Math.floor(size / 2);
            queue.push([top, left, half]);
            queue.push([top, left + half, half]);
            queue.push([top + half, left, half]);
            queue.push([top + half, left + half, half]);
        }
    }
    return root;
}

const grid = [[1, 1], [1, 0]];
const root = construct(grid);
console.log(root.val, root.isLeaf);
