#include <vector>
#include <queue>
using namespace std;

vector<vector<int>> merge(vector<vector<int>>& intervals) {
    priority_queue<pair<int, int>, vector<pair<int, int>>, greater<pair<int, int>>> pq;
    for (auto& interval : intervals) {
        pq.push({interval[0], interval[1]});
    }
    vector<vector<int>> merged;
    while (!pq.empty()) {
        auto [start, end] = pq.top(); pq.pop();
        if (!merged.empty() && start <= merged.back()[1]) {
            merged.back()[1] = max(merged.back()[1], end);
        } else {
            merged.push_back({start, end});
        }
    }
    return merged;
}
