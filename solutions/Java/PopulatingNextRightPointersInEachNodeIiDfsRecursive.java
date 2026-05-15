class Solution {
    public Node connect(Node root) {
        if (root == null) return null;
        dfs(root);
        return root;
    }
    
    private void dfs(Node node) {
        if (node == null) return;
        
        Node curr = node;
        while (curr != null) {
            Node next = curr.next;
            Node left = curr.left;
            Node right = curr.right;
            
            if (left != null && right != null) {
                left.next = right;
            }
            
            if (right != null && next != null) {
                right.next = next.left;
            }
            
            curr = next;
        }
        
        dfs(node.left);
        dfs(node.right);
    }
}

System.out.println("PopulatingNextRight Solution");
