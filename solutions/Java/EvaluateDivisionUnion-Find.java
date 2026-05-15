import java.util.*;

class Solution {
    public double[] calcEquation(List<List<String>> equations, double[] values, List<List<String>> queries) {
        Map<String, String> parent = new HashMap<>();
        Map<String, Double> ratio = new HashMap<>();
        
        for (int i = 0; i < equations.size(); i++) {
            String a = equations.get(i).get(0);
            String b = equations.get(i).get(1);
            union(a, b, values[i], parent, ratio);
        }
        
        double[] result = new double[queries.size()];
        for (int i = 0; i < queries.size(); i++) {
            String x = queries.get(i).get(0);
            String y = queries.get(i).get(1);
            if (!parent.containsKey(x) || !parent.containsKey(y) || !find(x, parent, ratio).equals(find(y, parent, ratio))) {
                result[i] = -1.0;
            } else {
                result[i] = ratio.get(x) / ratio.get(y);
            }
        }
        return result;
    }
    
    private void union(String a, String b, double val, Map<String, String> parent, Map<String, Double> ratio) {
        if (!parent.containsKey(a)) {
            parent.put(a, a);
            ratio.put(a, 1.0);
        }
        if (!parent.containsKey(b)) {
            parent.put(b, b);
            ratio.put(b, 1.0);
        }
        String pa = find(a, parent, ratio);
        String pb = find(b, parent, ratio);
        parent.put(pa, pb);
        ratio.put(pa, val * ratio.get(b) / ratio.get(a));
    }
    
    private String find(String x, Map<String, String> parent, Map<String, Double> ratio) {
        if (!parent.get(x).equals(x)) {
            String root = find(parent.get(x), parent, ratio);
            ratio.put(x, ratio.get(x) * ratio.get(parent.get(x)));
            parent.put(x, root);
        }
        return parent.get(x);
    }
}
