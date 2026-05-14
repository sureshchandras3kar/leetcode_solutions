class Solution {
    public int[] findOrder(int numCourses, int[][] prerequisites) {
        int[] state = new int[numCourses];
        int[][] graph = new int[numCourses][];
        int[] size = new int[numCourses];
        
        for (int[] pre : prerequisites) size[pre[0]]++;
        for (int i = 0; i < numCourses; i++) graph[i] = new int[size[i]];
        
        size = new int[numCourses];
        for (int[] pre : prerequisites) graph[pre[0]][size[pre[0]]++] = pre[1];
        
        int[] order = new int[numCourses];
        int[] idx = {numCourses - 1};
        
        for (int i = 0; i < numCourses; i++) {
            if (!dfs(graph, state, i, order, idx)) return new int[]{};
        }
        return order;
    }
    
    private boolean dfs(int[][] graph, int[] state, int course, int[] order, int[] idx) {
        if (state[course] == 1) return false;
        if (state[course] == 2) return true;
        
        state[course] = 1;
        for (int prereq : graph[course]) {
            if (!dfs(graph, state, prereq, order, idx)) return false;
        }
        state[course] = 2;
        order[idx[0]--] = course;
        return true;
    }
}
