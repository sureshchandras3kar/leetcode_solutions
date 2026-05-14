#include <vector>
using namespace std;

vector<vector<int>> insert(vector<vector<int>>& intervals, vector<int>& newInterval) {
    vector<vector<int>> result;
    int i = 0, new_start = newInterval[0], new_end = newInterval[1];
    while (i < intervals.size() && intervals[i][1] < new_start) {
        result.push_back(intervals[i++]);
    }
    while (i < intervals.size() && intervals[i][0] <= new_end) {
        new_start = min(new_start, intervals[i][0]);
        new_end = max(new_end, intervals[i][1]);
        i++;
    }
    result.push_back({new_start, new_end});
    while (i < intervals.size()) {
        result.push_back(intervals[i++]);
    }
    return result;
}
