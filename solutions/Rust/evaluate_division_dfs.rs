use std::collections::HashMap;

impl Solution {
    pub fn calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {
        let mut graph: HashMap<String, Vec<(String, f64)>> = HashMap::new();
        
        for (i, eq) in equations.iter().enumerate() {
            let a = &eq[0];
            let b = &eq[1];
            let val = values[i];
            
            graph.entry(a.clone()).or_insert_with(Vec::new).push((b.clone(), val));
            graph.entry(b.clone()).or_insert_with(Vec::new).push((a.clone(), 1.0 / val));
        }
        
        let mut result = Vec::new();
        for query in queries {
            let x = &query[0];
            let y = &query[1];
            
            if !graph.contains_key(x) || !graph.contains_key(y) {
                result.push(-1.0);
            } else {
                let mut visited = std::collections::HashSet::new();
                let res = dfs(x, y, 1.0, &graph, &mut visited);
                result.push(res);
            }
        }
        
        result
    }
}

fn dfs(curr: &String, target: &String, prod: f64, graph: &HashMap<String, Vec<(String, f64)>>, visited: &mut std::collections::HashSet<String>) -> f64 {
    if curr == target {
        return prod;
    }
    
    visited.insert(curr.clone());
    
    if let Some(neighbors) = graph.get(curr) {
        for (next, val) in neighbors {
            if !visited.contains(next) {
                let res = dfs(next, target, prod * val, graph, visited);
                if res != -1.0 {
                    return res;
                }
            }
        }
    }
    
    -1.0
}

struct Solution;

fn main() {
    println!("Solution");
}
