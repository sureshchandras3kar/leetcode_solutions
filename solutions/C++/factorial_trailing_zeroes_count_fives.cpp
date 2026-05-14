using namespace std;

int factorialTrailingZeroesCountFives(int n) {
    int count = 0;
    long long powerOf5 = 5;
    while (powerOf5 <= n) {
        count += n / powerOf5;
        powerOf5 *= 5;
    }
    return count;
}

int main() {
    cout << factorialTrailingZeroesCountFives(5) << endl;  // 1
    cout << factorialTrailingZeroesCountFives(25) << endl;  // 6
    return 0;
}
