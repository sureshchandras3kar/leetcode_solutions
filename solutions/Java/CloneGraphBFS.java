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
        Queue<Node> queue = new LinkedList<>();
        queue.offer(node);
        visited.put(node.val, new Node(node.val));
        
        while (!queue.isEmpty()) {
            Node curr = queue.poll();
            
            for (Node neighbor : curr.neighbors) {
                if (!visited.containsKey(neighbor.val)) {
                    visited.put(neighbor.val, new Node(neighbor.val));
                    queue.offer(neighbor);
                }
                visited.get(curr.val).neighbors.add(visited.get(neighbor.val));
            }
        }
        
        return visited.get(node.val);
    }
}
