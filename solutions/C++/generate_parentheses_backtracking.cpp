#include <vector>
#include <string>
using namespace std;

vector<string> generateParenthesis(int n) {
    vector<string> result;

    function<void(string, int, int)> backtrack = [&](string current, int left, int right) {
        if (current.length() == 2 * n) {
            result.push_back(current);
            return;
        }
        if (left < n) backtrack(current + "(", left + 1, right);
        if (right < left) backtrack(current + ")", left, right + 1);
    };

    backtrack("", 0, 0);
    return result;
}
