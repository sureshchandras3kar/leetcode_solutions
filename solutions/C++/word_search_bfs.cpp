#include <vector>
#include <string>
#include <queue>
#include <set>
using namespace std;

bool exist(vector<vector<char>>& board, string word) {
    for (int i = 0; i < board.size(); i++) {
        for (int j = 0; j < board[0].size(); j++) {
            if (board[i][j] == word[0]) {
                queue<tuple<int, int, int, set<pair<int, int>>>> q;
                set<pair<int, int>> visited;
                visited.insert({i, j});
                q.push({i, j, 1, visited});

                while (!q.empty()) {
                    auto [row, col, idx, vis] = q.front();
                    q.pop();
                    if (idx == word.length()) return true;

                    int dirs[4][2] = {{0,1}, {0,-1}, {1,0}, {-1,0}};
                    for (auto& d : dirs) {
                        int nr = row + d[0], nc = col + d[1];
                        if (nr >= 0 && nr < board.size() && nc >= 0 && nc < board[0].size() &&
                            board[nr][nc] == word[idx] && vis.find({nr, nc}) == vis.end()) {
                            auto newVis = vis;
                            newVis.insert({nr, nc});
                            q.push({nr, nc, idx + 1, newVis});
                        }
                    }
                }
            }
        }
    }
    return false;
}
