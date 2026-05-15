import java.util.*;

class Solution {
    public boolean canFinish(int numCourses, int[][] prerequisites) {
        List<Integer>[] graph = new List[numCourses];
        for (int i = 0; i < numCourses; i++) graph[i] = new ArrayList<>();
        
        for (int[] prereq : prerequisites) {
            graph[prereq[0]].add(prereq[1]);
        }
        
        int[] state = new int[numCourses];
        for (int i = 0; i < numCourses; i++) {
            if (!dfs(i, graph, state)) return false;
        }
        return true;
    }
    
    private boolean dfs(int course, List<Integer>[] graph, int[] state) {
        if (state[course] == 1) return false;
        if (state[course] == 2) return true;
        
        state[course] = 1;
        for (int prereq : graph[course]) {
            if (!dfs(prereq, graph, state)) return false;
        }
        state[course] = 2;
        return true;
    }
}

System.out.println(new Solution().canFinish(2, new int[][]{{1,0}}));
