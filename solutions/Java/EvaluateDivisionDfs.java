import java.util.*;

class Solution {
    public double[] calcEquation(List<List<String>> equations, double[] values, List<List<String>> queries) {
        Map<String, Map<String, Double>> graph = new HashMap<>();
        
        for (int i = 0; i < equations.size(); i++) {
            String a = equations.get(i).get(0);
            String b = equations.get(i).get(1);
            graph.putIfAbsent(a, new HashMap<>());
            graph.putIfAbsent(b, new HashMap<>());
            graph.get(a).put(b, values[i]);
            graph.get(b).put(a, 1.0 / values[i]);
        }
        
        double[] result = new double[queries.size()];
        for (int i = 0; i < queries.size(); i++) {
            String x = queries.get(i).get(0);
            String y = queries.get(i).get(1);
            if (!graph.containsKey(x) || !graph.containsKey(y)) {
                result[i] = -1.0;
            } else {
                result[i] = dfs(x, y, 1.0, graph, new HashSet<>());
            }
        }
        return result;
    }
    
    private double dfs(String curr, String target, double prod, Map<String, Map<String, Double>> graph, Set<String> visited) {
        if (curr.equals(target)) return prod;
        visited.add(curr);
        for (String next : graph.get(curr).keySet()) {
            if (!visited.contains(next)) {
                double res = dfs(next, target, prod * graph.get(curr).get(next), graph, visited);
                if (res != -1.0) return res;
            }
        }
        return -1.0;
    }
}
