import java.util.*;

class Solution {
    public boolean exist(char[][] board, String word) {
        if (board == null || board.length == 0 || word == null || word.length() == 0) {
            return false;
        }

        for (int i = 0; i < board.length; i++) {
            for (int j = 0; j < board[0].length; j++) {
                if (board[i][j] == word.charAt(0) && bfs(board, word, i, j)) {
                    return true;
                }
            }
        }

        return false;
    }

    private boolean bfs(char[][] board, String word, int startRow, int startCol) {
        Queue<int[]> queue = new LinkedList<>();
        Set<String> visited = new HashSet<>();
        queue.offer(new int[]{startRow, startCol, 0});
        visited.add(startRow + "," + startCol);

        int[][] directions = {{0, 1}, {0, -1}, {1, 0}, {-1, 0}};

        while (!queue.isEmpty()) {
            int[] current = queue.poll();
            int row = current[0], col = current[1], index = current[2];

            if (index == word.length() - 1) {
                return true;
            }

            for (int[] dir : directions) {
                int newRow = row + dir[0];
                int newCol = col + dir[1];
                String state = newRow + "," + newCol;

                if (newRow >= 0 && newRow < board.length &&
                    newCol >= 0 && newCol < board[0].length &&
                    board[newRow][newCol] == word.charAt(index + 1) &&
                    !visited.contains(state)) {

                    visited.add(state);
                    queue.offer(new int[]{newRow, newCol, index + 1});
                }
            }
        }

        return false;
    }
}
