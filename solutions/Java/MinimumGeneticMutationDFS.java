import java.util.*;

class Solution {
    public int minMutation(String startGene, String endGene, String[] bank) {
        Set<String> bankSet = new HashSet<>(Arrays.asList(bank));
        if (!bankSet.contains(endGene)) return -1;
        
        Set<String> visited = new HashSet<>();
        return dfs(startGene, endGene, bankSet, visited);
    }
    
    private int dfs(String curr, String end, Set<String> bank, Set<String> visited) {
        if (curr.equals(end)) return 0;
        visited.add(curr);
        int minSteps = Integer.MAX_VALUE;
        
        for (String next : getNeighbors(curr, bank, visited)) {
            int steps = dfs(next, end, bank, visited);
            if (steps != -1) minSteps = Math.min(minSteps, steps + 1);
        }
        
        return minSteps == Integer.MAX_VALUE ? -1 : minSteps;
    }
    
    private List<String> getNeighbors(String gene, Set<String> bank, Set<String> visited) {
        List<String> neighbors = new ArrayList<>();
        char[] genes = gene.toCharArray();
        char[] chars = {'A', 'C', 'G', 'T'};
        
        for (int i = 0; i < genes.length; i++) {
            char old = genes[i];
            for (char c : chars) {
                if (c != old) {
                    genes[i] = c;
                    String neighbor = new String(genes);
                    if (bank.contains(neighbor) && !visited.contains(neighbor)) {
                        neighbors.add(neighbor);
                    }
                }
            }
            genes[i] = old;
        }
        return neighbors;
    }
}
