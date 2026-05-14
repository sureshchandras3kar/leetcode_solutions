import java.util.*;

class Solution {
    public int minMutation(String startGene, String endGene, String[] bank) {
        Set<String> bankSet = new HashSet<>(Arrays.asList(bank));
        if (!bankSet.contains(endGene)) return -1;
        
        Map<String, Integer> memo = new HashMap<>();
        char[] chars = {'A', 'C', 'G', 'T'};
        return dfs(startGene, endGene, bankSet, memo, chars);
    }
    
    private int dfs(String gene, String endGene, Set<String> bankSet, Map<String, Integer> memo, char[] chars) {
        if (gene.equals(endGene)) return 0;
        if (memo.containsKey(gene)) return memo.get(gene);
        
        int result = Integer.MAX_VALUE;
        char[] arr = gene.toCharArray();
        for (int j = 0; j < arr.length; j++) {
            char old = arr[j];
            for (char c : chars) {
                if (c != old) {
                    arr[j] = c;
                    String newGene = new String(arr);
                    if (bankSet.contains(newGene)) {
                        int sub = dfs(newGene, endGene, bankSet, memo, chars);
                        if (sub != Integer.MAX_VALUE) {
                            result = Math.min(result, 1 + sub);
                        }
                    }
                }
            }
            arr[j] = old;
        }
        
        memo.put(gene, result);
        return result;
    }
}
