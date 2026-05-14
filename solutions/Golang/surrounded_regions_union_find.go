package main

type UnionFind struct {
    parent []int
    rank   []int
}

func NewUnionFind(n int) *UnionFind {
    uf := &UnionFind{
        parent: make([]int, n),
        rank:   make([]int, n),
    }
    for i := 0; i < n; i++ {
        uf.parent[i] = i
    }
    return uf
}

func (uf *UnionFind) Find(x int) int {
    if uf.parent[x] != x {
        uf.parent[x] = uf.Find(uf.parent[x])
    }
    return uf.parent[x]
}

func (uf *UnionFind) Unite(x, y int) {
    px, py := uf.Find(x), uf.Find(y)
    if px == py {
        return
    }
    if uf.rank[px] < uf.rank[py] {
        px, py = py, px
    }
    uf.parent[py] = px
    if uf.rank[px] == uf.rank[py] {
        uf.rank[px]++
    }
}

func solveUnionFind(board [][]byte) {
    if len(board) == 0 || len(board[0]) == 0 {
        return
    }
    
    rows, cols := len(board), len(board[0])
    uf := NewUnionFind(rows*cols + 1)
    boundary := rows * cols
    
    for i := 0; i < rows; i++ {
        for j := 0; j < cols; j++ {
            if board[i][j] == 'O' {
                if i == 0 || i == rows-1 || j == 0 || j == cols-1 {
                    uf.Unite(i*cols+j, boundary)
                } else {
                    if i > 0 && board[i-1][j] == 'O' {
                        uf.Unite(i*cols+j, (i-1)*cols+j)
                    }
                    if j > 0 && board[i][j-1] == 'O' {
                        uf.Unite(i*cols+j, i*cols+(j-1))
                    }
                }
            }
        }
    }
    
    for i := 0; i < rows; i++ {
        for j := 0; j < cols; j++ {
            if uf.Find(i*cols+j) != uf.Find(boundary) && board[i][j] == 'O' {
                board[i][j] = 'X'
            }
        }
    }
}
