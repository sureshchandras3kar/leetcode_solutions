import java.util.*;

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
        int n = grid.length;
        Queue<int[]> queue = new LinkedList<>();
        queue.offer(new int[]{0, 0, n});
        Node root = null;
        
        while (!queue.isEmpty()) {
            int[] task = queue.poll();
            int top = task[0], left = task[1], size = task[2];
            
            boolean allSame = true;
            int val = grid[top][left];
            for (int i = top; i < top + size && allSame; i++) {
                for (int j = left; j < left + size && allSame; j++) {
                    if (grid[i][j] != val) allSame = false;
                }
            }
            
            Node node = new Node(val == 1, allSame);
            if (root == null) root = node;
            
            if (!allSame) {
                int half = size / 2;
                queue.offer(new int[]{top, left, half});
                queue.offer(new int[]{top, left + half, half});
                queue.offer(new int[]{top + half, left, half});
                queue.offer(new int[]{top + half, left + half, half});
            }
        }
        return root;
    }
}

public class ConstructQuadTree_Iterative {
    public static void main(String[] args) {
        Solution sol = new Solution();
        int[][] grid = {{1, 1}, {1, 0}};
        Node root = sol.construct(grid);
        System.out.println(root.val + " " + root.isLeaf);
    }
}
