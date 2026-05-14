class Node {
    public boolean val;
    public boolean isLeaf;
    public Node topLeft;
    public Node topRight;
    public Node bottomLeft;
    public Node bottomRight;
    
    public Node(boolean val, boolean isLeaf) {
        this.val = val;
        this.isLeaf = isLeaf;
    }
}

class Solution {
    public Node construct(int[][] grid) {
        return dfs(grid, 0, 0, grid.length);
    }
    
    private Node dfs(int[][] grid, int top, int left, int size) {
        boolean allSame = true;
        int val = grid[top][left];
        
        for (int i = top; i < top + size && allSame; i++) {
            for (int j = left; j < left + size && allSame; j++) {
                if (grid[i][j] != val) allSame = false;
            }
        }
        
        if (allSame) return new Node(val == 1, true);
        
        int half = size / 2;
        Node node = new Node(true, false);
        node.topLeft = dfs(grid, top, left, half);
        node.topRight = dfs(grid, top, left + half, half);
        node.bottomLeft = dfs(grid, top + half, left, half);
        node.bottomRight = dfs(grid, top + half, left + half, half);
        return node;
    }
}

public class ConstructQuadTree_Recursive {
    public static void main(String[] args) {
        Solution sol = new Solution();
        int[][] grid = {{1, 1}, {1, 0}};
        Node root = sol.construct(grid);
        System.out.println(root.val + " " + root.isLeaf);
    }
}
