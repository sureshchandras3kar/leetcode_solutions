from collections import deque

def canFinish(numCourses, prerequisites):
    """BFS Kahn's algorithm for cycle detection."""
    indegree = [0] * numCourses
    graph = [[] for _ in range(numCourses)]
    
    for course, prereq in prerequisites:
        graph[prereq].append(course)
        indegree[course] += 1
    
    queue = deque([i for i in range(numCourses) if indegree[i] == 0])
    count = 0
    
    while queue:
        course = queue.popleft()
        count += 1
        for next_course in graph[course]:
            indegree[next_course] -= 1
            if indegree[next_course] == 0:
                queue.append(next_course)
    
    return count == numCourses
