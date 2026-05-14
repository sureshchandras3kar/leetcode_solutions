class Solution {
    public int snakesAndLadders(int[][] board) {
        int n = board.length;
        Map<Integer, Integer> memo = new HashMap<>();
        return dfs(board, 1, n, memo);
    }
    
    private int dfs(int[][] board, int pos, int n, Map<Integer, Integer> memo) {
        if (pos == n * n) return 0;
        if (memo.containsKey(pos)) return memo.get(pos);
        
        int result = Integer.MAX_VALUE;
        for (int i = 1; i <= 6; i++) {
            int nextPos = pos + i;
            if (nextPos > n * n) break;
            
            int[] coords = getCoords(nextPos, n);
            if (board[coords[0]][coords[1]] != -1) {
                nextPos = board[coords[0]][coords[1]];
            }
            
            int sub = dfs(board, nextPos, n, memo);
            if (sub != Integer.MAX_VALUE) {
                result = Math.min(result, 1 + sub);
            }
        }
        
        memo.put(pos, result);
        return result;
    }
    
    private int[] getCoords(int pos, int n) {
        pos--;
        int row = n - 1 - pos / n;
        int col = (n - 1 - row) % 2 == 0 ? pos % n : n - 1 - pos % n;
        return new int[]{row, col};
    }
}
