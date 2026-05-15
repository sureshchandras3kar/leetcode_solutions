class Solution {
    public int snakesAndLadders(int[][] board) {
        int n = board.length;
        return dfs(1, board, n, new boolean[n * n + 1]);
    }
    
    private int dfs(int pos, int[][] board, int n, boolean[] visited) {
        if (pos == n * n) return 0;
        visited[pos] = true;
        int minMoves = Integer.MAX_VALUE;
        
        for (int next = pos + 1; next <= Math.min(pos + 6, n * n); next++) {
            int[] cell = getCell(next, n);
            int destination = board[cell[0]][cell[1]];
            if (destination != -1) next = destination;
            
            if (!visited[next]) {
                int moves = dfs(next, board, n, visited);
                if (moves != Integer.MAX_VALUE) {
                    minMoves = Math.min(minMoves, moves + 1);
                }
            }
        }
        
        return minMoves;
    }
    
    private int[] getCell(int pos, int n) {
        int row = (pos - 1) / n;
        int col = (pos - 1) % n;
        if (row % 2 == 1) col = n - 1 - col;
        return new int[]{n - 1 - row, col};
    }
}
