import java.util.*;

public class Bfs {
    public boolean exist(char[][] board, String word) {
        for (int i = 0; i < board.length; i++) {
            for (int j = 0; j < board[0].length; j++) {
                if (bfs(board, word, i, j)) return true;
            }
        }
        return false;
    }

    private boolean bfs(char[][] board, String word, int startRow, int startCol) {
        Queue<State> queue = new LinkedList<>();
        queue.offer(new State(startRow, startCol, 0, new HashSet<>()));

        while (!queue.isEmpty()) {
            State state = queue.poll();
            if (state.index == word.length()) return true;

            int[][] dirs = {{0,1}, {0,-1}, {1,0}, {-1,0}};
            for (int[] d : dirs) {
                int nr = state.row + d[0], nc = state.col + d[1];
                if (nr >= 0 && nr < board.length && nc >= 0 && nc < board[0].length &&
                    board[nr][nc] == word.charAt(state.index) && !state.visited.contains(nr * 100 + nc)) {
                    HashSet<Integer> newVis = new HashSet<>(state.visited);
                    newVis.add(nr * 100 + nc);
                    queue.offer(new State(nr, nc, state.index + 1, newVis));
                }
            }
        }
        return false;
    }

    static class State {
        int row, col, index;
        Set<Integer> visited;
        State(int r, int c, int i, Set<Integer> v) {
            row = r; col = c; index = i; visited = v;
        }
    }
}
