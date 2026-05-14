import java.util.*;

class TrieNode {
    Map<Character, TrieNode> children = new HashMap<>();
    String word = null;
}

class Solution {
    private List<String> result = new ArrayList<>();

    public List<String> findWords(char[][] board, String[] words) {
        TrieNode root = new TrieNode();
        for (String word : words) {
            TrieNode node = root;
            for (char c : word.toCharArray()) {
                node.children.putIfAbsent(c, new TrieNode());
                node = node.children.get(c);
            }
            node.word = word;
        }

        for (int i = 0; i < board.length; i++) {
            for (int j = 0; j < board[0].length; j++) {
                backtrack(board, i, j, root);
            }
        }
        return result;
    }

    private void backtrack(char[][] board, int i, int j, TrieNode node) {
        if (i < 0 || i >= board.length || j < 0 || j >= board[0].length) {
            return;
        }

        char ch = board[i][j];
        if (!node.children.containsKey(ch)) {
            return;
        }

        TrieNode next = node.children.get(ch);
        if (next.word != null) {
            result.add(next.word);
            next.word = null;
        }

        board[i][j] = '#';
        int[][] dirs = {{0, 1}, {1, 0}, {0, -1}, {-1, 0}};
        for (int[] dir : dirs) {
            backtrack(board, i + dir[0], j + dir[1], next);
        }
        board[i][j] = ch;
    }
}

public class WordSearchII_Backtrack {
    public static void main(String[] args) {
        Solution sol = new Solution();
        char[][] board = {
            {'o', 'a', 'a', 'n'},
            {'e', 't', 'a', 'e'},
            {'i', 'h', 'k', 'r'},
            {'i', 'f', 'l', 'v'}
        };
        String[] words = {"oath", "pea", "eat", "rain"};
        System.out.println(sol.findWords(board, words));
    }
}
