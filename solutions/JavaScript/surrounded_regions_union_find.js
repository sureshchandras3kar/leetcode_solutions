class UnionFind {
    constructor(n) {
        this.parent = Array.from({length: n}, (_, i) => i);
        this.rank = Array(n).fill(0);
    }
    
    find(x) {
        if (this.parent[x] !== x) {
            this.parent[x] = this.find(this.parent[x]);
        }
        return this.parent[x];
    }
    
    unite(x, y) {
        let px = this.find(x), py = this.find(y);
        if (px === py) return;
        if (this.rank[px] < this.rank[py]) [px, py] = [py, px];
        this.parent[py] = px;
        if (this.rank[px] === this.rank[py]) this.rank[px]++;
    }
}

function solveUnionFind(board) {
    if (!board || board.length === 0) return;
    
    const rows = board.length, cols = board[0].length;
    const uf = new UnionFind(rows * cols + 1);
    const boundary = rows * cols;
    
    for (let i = 0; i < rows; i++) {
        for (let j = 0; j < cols; j++) {
            if (board[i][j] === 'O') {
                if (i === 0 || i === rows - 1 || j === 0 || j === cols - 1) {
                    uf.unite(i * cols + j, boundary);
                } else {
                    if (i > 0 && board[i-1][j] === 'O') uf.unite(i*cols+j, (i-1)*cols+j);
                    if (j > 0 && board[i][j-1] === 'O') uf.unite(i*cols+j, i*cols+(j-1));
                }
            }
        }
    }
    
    for (let i = 0; i < rows; i++) {
        for (let j = 0; j < cols; j++) {
            if (uf.find(i * cols + j) !== uf.find(boundary) && board[i][j] === 'O') {
                board[i][j] = 'X';
            }
        }
    }
}
