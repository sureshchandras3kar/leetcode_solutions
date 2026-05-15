#include <vector>
#include <unordered_map>
#include <string>
using namespace std;

class Solution {
public:
    vector<double> calcEquation(vector<vector<string>>& equations, vector<double>& values, vector<vector<string>>& queries) {
        unordered_map<string, pair<string, double>> parent;
        
        for (int i = 0; i < equations.size(); i++) {
            unite(equations[i][0], equations[i][1], values[i], parent);
        }
        
        vector<double> result;
        for (const auto& query : queries) {
            string x = query[0], y = query[1];
            if (parent.find(x) == parent.end() || parent.find(y) == parent.end()) {
                result.push_back(-1.0);
            } else {
                auto [px, rx] = find(x, parent);
                auto [py, ry] = find(y, parent);
                if (px != py) {
                    result.push_back(-1.0);
                } else {
                    result.push_back(rx / ry);
                }
            }
        }
        return result;
    }
    
private:
    void unite(const string& a, const string& b, double val, unordered_map<string, pair<string, double>>& parent) {
        if (parent.find(a) == parent.end()) parent[a] = {a, 1.0};
        if (parent.find(b) == parent.end()) parent[b] = {b, 1.0};
        
        auto [pa, ra] = find(a, parent);
        auto [pb, rb] = find(b, parent);
        parent[pa] = {pb, val * rb / ra};
    }
    
    pair<string, double> find(const string& x, unordered_map<string, pair<string, double>>& parent) {
        if (parent[x].first != x) {
            auto [root, ratio] = find(parent[x].first, parent);
            parent[x].second *= ratio;
            parent[x].first = root;
        }
        return {parent[x].first, parent[x].second};
    }
};
