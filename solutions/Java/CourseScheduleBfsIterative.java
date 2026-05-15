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
            if (inDegree[i] == 0) queue.add(i);
        }
        
        int processed = 0;
        while (!queue.isEmpty()) {
            int course = queue.remove();
            processed++;
            
            for (int nextCourse : graph[course]) {
                inDegree[nextCourse]--;
                if (inDegree[nextCourse] == 0) {
                    queue.add(nextCourse);
                }
            }
        }
        
        return processed == numCourses;
    }
}

System.out.println(new Solution().canFinish(2, new int[][]{{1,0}}));
