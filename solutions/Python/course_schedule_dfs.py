def canFinish(numCourses, prerequisites):
    """DFS topological sort to detect cycle."""
    graph = [[] for _ in range(numCourses)]
    for course, prereq in prerequisites:
        graph[course].append(prereq)
    
    state = [0] * numCourses  # 0: unvisited, 1: visiting, 2: visited
    
    def has_cycle(course):
        if state[course] == 1:
            return True  # Cycle detected
        if state[course] == 2:
            return False  # Already processed
        
        state[course] = 1
        for prereq in graph[course]:
            if has_cycle(prereq):
                return True
        state[course] = 2
        return False
    
    for i in range(numCourses):
        if has_cycle(i):
            return False
    return True
