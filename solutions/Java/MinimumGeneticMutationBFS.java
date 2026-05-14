import java.util.*;

class Solution {
    public int minMutation(String startGene, String endGene, String[] bank) {
        Set<String> bankSet = new HashSet<>(Arrays.asList(bank));
        if (!bankSet.contains(endGene)) return -1;
        
        Queue<String> queue = new LinkedList<>();
        Set<String> visited = new HashSet<>();
        queue.offer(startGene);
        visited.add(startGene);
        
        int mutations = 0;
        char[] chars = {'A', 'C', 'G', 'T'};
        
        while (!queue.isEmpty()) {
            int size = queue.size();
            for (int i = 0; i < size; i++) {
                String gene = queue.poll();
                if (gene.equals(endGene)) return mutations;
                
                char[] arr = gene.toCharArray();
                for (int j = 0; j < arr.length; j++) {
                    char old = arr[j];
                    for (char c : chars) {
                        if (c != old) {
                            arr[j] = c;
                            String newGene = new String(arr);
                            if (bankSet.contains(newGene) && !visited.contains(newGene)) {
                                visited.add(newGene);
                                queue.offer(newGene);
                            }
                        }
                    }
                    arr[j] = old;
                }
            }
            mutations++;
        }
        return -1;
    }
}
