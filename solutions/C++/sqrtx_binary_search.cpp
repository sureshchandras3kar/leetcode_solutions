using namespace std;

int sqrtxBinarySearch(int x) {
    if (x < 2) return x;

    long long left = 2, right = x / 2;
    while (left <= right) {
        long long mid = (left + right) / 2;
        if (mid * mid == x) {
            return mid;
        } else if (mid * mid < x) {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    return right;
}

int main() {
    cout << sqrtxBinarySearch(4) << endl;  // 2
    cout << sqrtxBinarySearch(8) << endl;  // 2
    return 0;
}
