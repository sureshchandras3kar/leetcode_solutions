#include <vector>
#include <algorithm>
using namespace std;

int findMinArrowShots(vector<vector<int>>& points) {
    if (points.empty()) return 0;
    sort(points.begin(), points.end());
    int arrows = 1;
    long long last_end = points[0][1];
    for (int i = 1; i < points.size(); i++) {
        if ((long long)points[i][0] > last_end) {
            arrows++;
            last_end = points[i][1];
        }
    }
    return arrows;
}
