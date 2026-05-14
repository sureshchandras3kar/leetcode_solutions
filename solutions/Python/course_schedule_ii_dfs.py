def findOrder(numCourses, prerequisites):
    """DFS topological sort to get course order."""
    graph = [[] for _ in range(numCourses)]
    for course, prereq in prerequisites:
        graph[course].append(prereq)
    
    state = [0] * numCourses
    order = []
    
    def dfs(course):
        if state[course] == 1:
            return False
        if state[course] == 2:
            return True
        
        state[course] = 1
        for prereq in graph[course]:
            if not dfs(prereq):
                return False
        state[course] = 2
        order.append(course)
        return True
    
    for i in range(numCourses):
        if not dfs(i):
            return []
    return order
