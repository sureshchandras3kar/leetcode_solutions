def calcEquation(equations, values, queries):
    parent = {}
    rank = {}
    
    def find(x):
        if x not in parent:
            parent[x] = x
            rank[x] = 0
        if parent[x] != x:
            parent[x] = find(parent[x])
        return parent[x]
    
    def union(a, b, val):
        pa, pb = find(a), find(b)
        if pa == pb:
            return
        parent[pb] = pa
        rank[pa] = max(rank[pa], rank[pb] + 1)
    
    for (a, b), val in zip(equations, values):
        union(a, b, val)
    
    result = []
    for x, y in queries:
        if find(x) != find(y):
            result.append(-1.0)
        else:
            result.append(1.0)
    
    return result
