class Solution {
public:
    vector<double> calcEquation(vector<vector<string>>& equations, vector<double>& values, vector<vector<string>>& queries) {
        unordered_map<string, pair<string, double>> parent;
        
        for (int i = 0; i < equations.size(); i++) {
            unite(parent, equations[i][0], equations[i][1], values[i]);
        }
        
        vector<double> result;
        for (auto& query : queries) {
            if (parent.find(query[0]) == parent.end() || parent.find(query[1]) == parent.end()) {
                result.push_back(-1.0);
            } else {
                auto [root0, weight0] = find(parent, query[0]);
                auto [root1, weight1] = find(parent, query[1]);
                if (root0 != root1) {
                    result.push_back(-1.0);
                } else {
                    result.push_back(weight0 / weight1);
                }
            }
        }
        return result;
    }
    
private:
    pair<string, double> find(unordered_map<string, pair<string, double>>& parent, string x) {
        if (parent.find(x) == parent.end()) {
            parent[x] = {x, 1.0};
        }
        if (parent[x].first == x) return {x, 1.0};
        auto [root, weight] = find(parent, parent[x].first);
        parent[x] = {root, parent[x].second * weight};
        return parent[x];
    }
    
    void unite(unordered_map<string, pair<string, double>>& parent, string x, string y, double val) {
        auto [rootX, weightX] = find(parent, x);
        auto [rootY, weightY] = find(parent, y);
        parent[rootX] = {rootY, val * weightY / weightX};
    }
};
