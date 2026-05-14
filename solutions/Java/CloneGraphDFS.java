class Node {
    public int val;
    public List<Node> neighbors = new ArrayList<Node>();
    public Node() { }
    public Node(int _val) { val = _val; }
}

class Solution {
    public Node cloneGraph(Node node) {
        if (node == null) return null;
        Map<Integer, Node> visited = new HashMap<>();
        return dfs(node, visited);
    }
    
    private Node dfs(Node node, Map<Integer, Node> visited) {
        if (visited.containsKey(node.val)) {
            return visited.get(node.val);
        }
        
        Node cloned = new Node(node.val);
        visited.put(node.val, cloned);
        
        for (Node neighbor : node.neighbors) {
            cloned.neighbors.add(dfs(neighbor, visited));
        }
        
        return cloned;
    }
}
