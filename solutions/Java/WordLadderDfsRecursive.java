import java.util.*;

class Solution {
    public int ladderLength(String beginWord, String endWord, List<String> wordList) {
        Set<String> wordSet = new HashSet<>(wordList);
        if (!wordSet.contains(endWord)) return 0;
        
        Set<String> visited = new HashSet<>();
        int[] result = {Integer.MAX_VALUE};
        dfs(beginWord, endWord, wordSet, visited, 1, result);
        
        return result[0] == Integer.MAX_VALUE ? 0 : result[0];
    }
    
    private void dfs(String curr, String end, Set<String> wordSet, Set<String> visited, int level, int[] result) {
        if (curr.equals(end)) {
            result[0] = Math.min(result[0], level);
            return;
        }
        
        for (String next : getNeighbors(curr, wordSet, visited)) {
            visited.add(next);
            dfs(next, end, wordSet, visited, level + 1, result);
            visited.remove(next);
        }
    }
    
    private List<String> getNeighbors(String word, Set<String> wordSet, Set<String> visited) {
        List<String> neighbors = new ArrayList<>();
        char[] chars = word.toCharArray();
        
        for (int i = 0; i < chars.length; i++) {
            char old = chars[i];
            for (char c = 'a'; c <= 'z'; c++) {
                if (c != old) {
                    chars[i] = c;
                    String neighbor = new String(chars);
                    if (wordSet.contains(neighbor) && !visited.contains(neighbor)) {
                        neighbors.add(neighbor);
                    }
                }
            }
            chars[i] = old;
        }
        
        return neighbors;
    }
}
