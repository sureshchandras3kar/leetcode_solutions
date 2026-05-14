class UnionFind {
public:
    vector<int> parent, rank;
    
    UnionFind(int n) {
        parent.resize(n);
        rank.resize(n, 0);
        for (int i = 0; i < n; i++) parent[i] = i;
    }
    
    int find(int x) {
        if (parent[x] != x) parent[x] = find(parent[x]);
        return parent[x];
    }
    
    void unite(int x, int y) {
        int px = find(x), py = find(y);
        if (px == py) return;
        if (rank[px] < rank[py]) swap(px, py);
        parent[py] = px;
        if (rank[px] == rank[py]) rank[px]++;
    }
};

class SurroundedRegionsUnionFind {
public:
    void solve(vector<vector<char>>& board) {
        if (board.empty() || board[0].empty()) return;

        int rows = board.size();
        int cols = board[0].size();
        UnionFind uf(rows * cols + 1);
        int boundary = rows * cols;

        for (int i = 0; i < rows; i++) {
            for (int j = 0; j < cols; j++) {
                if (board[i][j] == 'O') {
                    if (i == 0 || i == rows - 1 || j == 0 || j == cols - 1) {
                        uf.unite(i * cols + j, boundary);
                    } else {
                        if (i > 0 && board[i-1][j] == 'O') uf.unite(i*cols+j, (i-1)*cols+j);
                        if (j > 0 && board[i][j-1] == 'O') uf.unite(i*cols+j, i*cols+(j-1));
                    }
                }
            }
        }

        for (int i = 0; i < rows; i++) {
            for (int j = 0; j < cols; j++) {
                if (uf.find(i * cols + j) != uf.find(boundary) && board[i][j] == 'O') {
                    board[i][j] = 'X';
                }
            }
        }
    }
};
