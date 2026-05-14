def minMutation(startGene, endGene, bank):
    """DFS with memoization to find minimum mutations."""
    if endGene not in bank:
        return -1
    
    bank_set = set(bank)
    memo = {}
    chars = ['A', 'C', 'G', 'T']
    
    def dfs(gene):
        if gene == endGene:
            return 0
        if gene in memo:
            return memo[gene]
        
        result = float('inf')
        for i in range(len(gene)):
            for char in chars:
                if char != gene[i]:
                    new_gene = gene[:i] + char + gene[i+1:]
                    if new_gene in bank_set:
                        result = min(result, 1 + dfs(new_gene))
        
        memo[gene] = result
        return result
    
    ans = dfs(startGene)
    return ans if ans != float('inf') else -1
