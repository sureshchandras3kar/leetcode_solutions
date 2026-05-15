import java.util.*;

class Solution {
    public boolean canFinish(int numCourses, int[][] prerequisites) {
        List<Integer>[] graph = new List[numCourses];
        int[] inDegree = new int[numCourses];
        
        for (int i = 0; i < numCourses; i++) graph[i] = new ArrayList<>();
        
        for (int[] prereq : prerequisites) {
            graph[prereq[1]].add(prereq[0]);
            inDegree[prereq[0]]++;
        }
        
        Queue<Integer> queue = new LinkedList<>();
        for (int i = 0; i < numCourses; i++) {
            if (inDegree[i] == 0) queue.offer(i);
        }
        
        int count = 0;
        while (!queue.isEmpty()) {
            int course = queue.poll();
            count++;
            for (int next : graph[course]) {
                inDegree[next]--;
                if (inDegree[next] == 0) queue.offer(next);
            }
        }
        
        return count == numCourses;
    }
}

System.out.println(new Solution().canFinish(2, new int[][]{{1,0}}));
