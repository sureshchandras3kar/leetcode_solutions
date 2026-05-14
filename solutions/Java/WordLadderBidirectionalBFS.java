import java.util.*;

class Solution {
    public int ladderLength(String beginWord, String endWord, List<String> wordList) {
        Set<String> wordSet = new HashSet<>(wordList);
        if (!wordSet.contains(endWord)) return 0;
        
        Set<String> beginSet = new HashSet<>(), endSet = new HashSet<>();
        beginSet.add(beginWord);
        endSet.add(endWord);
        
        return bfs(beginSet, endSet, wordSet, 1);
    }
    
    private int bfs(Set<String> beginSet, Set<String> endSet, Set<String> wordSet, int length) {
        if (beginSet.isEmpty()) return 0;
        if (beginSet.size() > endSet.size()) {
            return bfs(endSet, beginSet, wordSet, length);
        }
        
        wordSet.removeAll(beginSet);
        Set<String> nextLevel = new HashSet<>();
        
        for (String word : beginSet) {
            char[] arr = word.toCharArray();
            for (int i = 0; i < arr.length; i++) {
                char old = arr[i];
                for (char c = 'a'; c <= 'z'; c++) {
                    if (c != old) {
                        arr[i] = c;
                        String newWord = new String(arr);
                        if (endSet.contains(newWord)) return length + 1;
                        if (wordSet.contains(newWord)) nextLevel.add(newWord);
                    }
                }
                arr[i] = old;
            }
        }
        
        return bfs(nextLevel, endSet, wordSet, length + 1);
    }
}
