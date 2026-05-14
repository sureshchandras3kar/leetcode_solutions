class Solution {
    public boolean canFinish(int numCourses, int[][] prerequisites) {
        int[] state = new int[numCourses];
        int[][] graph = new int[numCourses][];
        int[] size = new int[numCourses];
        
        for (int[] pre : prerequisites) {
            size[pre[1]]++;
        }
        
        for (int i = 0; i < numCourses; i++) {
            graph[i] = new int[size[i]];
        }
        
        size = new int[numCourses];
        for (int[] pre : prerequisites) {
            graph[pre[1]][size[pre[1]]++] = pre[0];
        }
        
        for (int i = 0; i < numCourses; i++) {
            if (hasCycle(graph, state, i)) return false;
        }
        return true;
    }
    
    private boolean hasCycle(int[][] graph, int[] state, int course) {
        if (state[course] == 1) return true;
        if (state[course] == 2) return false;
        
        state[course] = 1;
        for (int prereq : graph[course]) {
            if (hasCycle(graph, state, prereq)) return true;
        }
        state[course] = 2;
        return false;
    }
}
