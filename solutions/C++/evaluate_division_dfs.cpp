class Solution {
public:
    vector<double> calcEquation(vector<vector<string>>& equations, vector<double>& values, vector<vector<string>>& queries) {
        unordered_map<string, vector<pair<string, double>>> graph;
        for (int i = 0; i < equations.size(); i++) {
            graph[equations[i][0]].push_back({equations[i][1], values[i]});
            graph[equations[i][1]].push_back({equations[i][0], 1.0 / values[i]});
        }
        
        vector<double> result;
        for (auto& query : queries) {
            unordered_set<string> visited;
            result.push_back(dfs(graph, query[0], query[1], visited));
        }
        return result;
    }
    
private:
    double dfs(unordered_map<string, vector<pair<string, double>>>& graph, string start, string end, unordered_set<string>& visited) {
        if (graph.find(start) == graph.end() || graph.find(end) == graph.end()) return -1.0;
        if (start == end) return 1.0;
        
        visited.insert(start);
        for (auto& [next, weight] : graph[start]) {
            if (visited.find(next) == visited.end()) {
                double result = dfs(graph, next, end, visited);
                if (result != -1.0) return result * weight;
            }
        }
        return -1.0;
    }
};
