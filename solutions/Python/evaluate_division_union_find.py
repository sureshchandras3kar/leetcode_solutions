class UnionFind:
    def __init__(self):
        self.parent = {}
        self.weight = {}
    
    def find(self, x):
        if x not in self.parent:
            self.parent[x] = x
            self.weight[x] = 1.0
        if self.parent[x] != x:
            py = self.parent[x]
            self.parent[x] = self.find(py)
            self.weight[x] *= self.weight[py]
        return self.parent[x]
    
    def union(self, a, b, val):
        pya, pyb = self.find(a), self.find(b)
        if pya != pyb:
            self.parent[pya] = pyb
            self.weight[pya] = val * self.weight[b] / self.weight[a]

def calcEquation(equations, values, queries):
    uf = UnionFind()
    for (a, b), val in zip(equations, values):
        uf.union(a, b, val)
    
    results = []
    for a, b in queries:
        if a not in uf.parent or b not in uf.parent:
            results.append(-1.0)
        elif uf.find(a) != uf.find(b):
            results.append(-1.0)
        else:
            results.append(uf.weight[a] / uf.weight[b])
    
    return results
