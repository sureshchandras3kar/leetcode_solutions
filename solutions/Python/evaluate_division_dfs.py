def calcEquation(equations, values, queries):
    graph = {}
    for (a, b), val in zip(equations, values):
        if a not in graph:
            graph[a] = []
        if b not in graph:
            graph[b] = []
        graph[a].append((b, val))
        graph[b].append((a, 1.0 / val))
    
    def dfs(start, end, visited):
        if start not in graph or end not in graph:
            return -1.0
        if start == end:
            return 1.0
        
        visited.add(start)
        for neighbor, weight in graph[start]:
            if neighbor not in visited:
                result = dfs(neighbor, end, visited)
                if result != -1.0:
                    return result * weight
        return -1.0
    
    return [dfs(a, b, set()) for a, b in queries]
