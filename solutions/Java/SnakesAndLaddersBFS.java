import java.util.*;

class Solution {
    public int snakesAndLadders(int[][] board) {
        int n = board.length;
        Queue<Integer> queue = new LinkedList<>();
        Set<Integer> visited = new HashSet<>();
        queue.offer(1);
        visited.add(1);
        int moves = 0;
        
        while (!queue.isEmpty()) {
            int size = queue.size();
            for (int i = 0; i < size; i++) {
                int pos = queue.poll();
                if (pos == n * n) return moves;
                
                for (int next = pos + 1; next <= Math.min(pos + 6, n * n); next++) {
                    int[] cell = getCell(next, n);
                    int destination = board[cell[0]][cell[1]];
                    if (destination != -1) next = destination;
                    
                    if (!visited.contains(next)) {
                        visited.add(next);
                        queue.offer(next);
                    }
                }
            }
            moves++;
        }
        return -1;
    }
    
    private int[] getCell(int pos, int n) {
        int row = (pos - 1) / n;
        int col = (pos - 1) % n;
        if (row % 2 == 1) col = n - 1 - col;
        return new int[]{n - 1 - row, col};
    }
}
