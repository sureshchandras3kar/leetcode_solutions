struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }
    
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }
    
    fn unite(&mut self, x: usize, y: usize) {
        let mut px = self.find(x);
        let mut py = self.find(y);
        if px == py { return; }
        if self.rank[px] < self.rank[py] {
            std::mem::swap(&mut px, &mut py);
        }
        self.parent[py] = px;
        if self.rank[px] == self.rank[py] {
            self.rank[px] += 1;
        }
    }
}

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        if board.is_empty() || board[0].is_empty() {
            return;
        }
        
        let rows = board.len();
        let cols = board[0].len();
        let mut uf = UnionFind::new(rows * cols + 1);
        let boundary = rows * cols;
        
        for i in 0..rows {
            for j in 0..cols {
                if board[i][j] == 'O' {
                    if i == 0 || i == rows - 1 || j == 0 || j == cols - 1 {
                        uf.unite(i * cols + j, boundary);
                    } else {
                        if i > 0 && board[i-1][j] == 'O' {
                            uf.unite(i*cols+j, (i-1)*cols+j);
                        }
                        if j > 0 && board[i][j-1] == 'O' {
                            uf.unite(i*cols+j, i*cols+(j-1));
                        }
                    }
                }
            }
        }
        
        for i in 0..rows {
            for j in 0..cols {
                if uf.find(i * cols + j) != uf.find(boundary) && board[i][j] == 'O' {
                    board[i][j] = 'X';
                }
            }
        }
    }
}

pub struct Solution;
