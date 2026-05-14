class UnionFind:
    def __init__(self, n: int):
        self.parent = list(range(n))
        self.rank = [0] * n

    def find(self, x: int) -> int:
        if self.parent[x] != x:
            self.parent[x] = self.find(self.parent[x])
        return self.parent[x]

    def union(self, x: int, y: int) -> None:
        px, py = self.find(x), self.find(y)
        if px == py:
            return
        if self.rank[px] < self.rank[py]:
            px, py = py, px
        self.parent[py] = px
        if self.rank[px] == self.rank[py]:
            self.rank[px] += 1

def solve_union_find(board: list[list[str]]) -> None:
    """
    Union-Find approach - unite boundary with border-connected O's.
    Time: O(m * n * α(n))
    Space: O(m * n)
    """
    if not board or not board[0]:
        return

    rows, cols = len(board), len(board[0])
    uf = UnionFind(rows * cols + 1)
    boundary = rows * cols

    for i in range(rows):
        for j in range(cols):
            if board[i][j] == 'O':
                if i == 0 or i == rows - 1 or j == 0 or j == cols - 1:
                    uf.union(i * cols + j, boundary)
                else:
                    if i > 0 and board[i - 1][j] == 'O':
                        uf.union(i * cols + j, (i - 1) * cols + j)
                    if j > 0 and board[i][j - 1] == 'O':
                        uf.union(i * cols + j, i * cols + (j - 1))

    for i in range(rows):
        for j in range(cols):
            if uf.find(i * cols + j) != uf.find(boundary) and board[i][j] == 'O':
                board[i][j] = 'X'
