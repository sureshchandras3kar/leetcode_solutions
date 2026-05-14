import java.util.*;

class Solution {
    public int snakesAndLadders(int[][] board) {
        int n = board.length;
        Queue<int[]> queue = new LinkedList<>();
        queue.offer(new int[]{1, 0});
        boolean[] visited = new boolean[n * n + 1];
        visited[1] = true;
        
        while (!queue.isEmpty()) {
            int[] curr = queue.poll();
            int pos = curr[0], moves = curr[1];
            
            if (pos == n * n) return moves;
            
            for (int i = 1; i <= 6; i++) {
                int nextPos = pos + i;
                if (nextPos > n * n) break;
                
                int[] coords = getCoords(nextPos, n);
                if (board[coords[0]][coords[1]] != -1) {
                    nextPos = board[coords[0]][coords[1]];
                }
                
                if (!visited[nextPos]) {
                    visited[nextPos] = true;
                    queue.offer(new int[]{nextPos, moves + 1});
                }
            }
        }
        return -1;
    }
    
    private int[] getCoords(int pos, int n) {
        pos--;
        int row = n - 1 - pos / n;
        int col = (n - 1 - row) % 2 == 0 ? pos % n : n - 1 - pos % n;
        return new int[]{row, col};
    }
}
