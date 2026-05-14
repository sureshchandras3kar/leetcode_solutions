from collections import deque

def minMutation(startGene, endGene, bank):
    """BFS to find minimum mutations."""
    if endGene not in bank:
        return -1
    
    bank_set = set(bank)
    queue = deque([(startGene, 0)])
    visited = {startGene}
    chars = ['A', 'C', 'G', 'T']
    
    while queue:
        gene, mutations = queue.popleft()
        if gene == endGene:
            return mutations
        
        for i in range(len(gene)):
            for char in chars:
                if char != gene[i]:
                    new_gene = gene[:i] + char + gene[i+1:]
                    if new_gene in bank_set and new_gene not in visited:
                        visited.add(new_gene)
                        queue.append((new_gene, mutations + 1))
    
    return -1
