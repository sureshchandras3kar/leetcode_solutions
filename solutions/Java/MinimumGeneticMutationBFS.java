import java.util.*;

class Solution {
    public int minMutation(String startGene, String endGene, String[] bank) {
        Set<String> bankSet = new HashSet<>(Arrays.asList(bank));
        if (!bankSet.contains(endGene)) return -1;
        
        Queue<String> queue = new LinkedList<>();
        queue.offer(startGene);
        Set<String> visited = new HashSet<>();
        visited.add(startGene);
        int steps = 0;
        
        while (!queue.isEmpty()) {
            int size = queue.size();
            for (int i = 0; i < size; i++) {
                String curr = queue.poll();
                if (curr.equals(endGene)) return steps;
                
                for (String next : getNeighbors(curr, bankSet)) {
                    if (!visited.contains(next)) {
                        visited.add(next);
                        queue.offer(next);
                    }
                }
            }
            steps++;
        }
        return -1;
    }
    
    private List<String> getNeighbors(String gene, Set<String> bank) {
        List<String> neighbors = new ArrayList<>();
        char[] genes = gene.toCharArray();
        char[] chars = {'A', 'C', 'G', 'T'};
        
        for (int i = 0; i < genes.length; i++) {
            char old = genes[i];
            for (char c : chars) {
                if (c != old) {
                    genes[i] = c;
                    String neighbor = new String(genes);
                    if (bank.contains(neighbor)) neighbors.add(neighbor);
                }
            }
            genes[i] = old;
        }
        return neighbors;
    }
}
