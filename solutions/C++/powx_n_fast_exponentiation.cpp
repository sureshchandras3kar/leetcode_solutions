using namespace std;

double powxnFastExponentiation(double x, int n) {
    if (n == 0) return 1.0;

    long long N = n;
    if (N < 0) {
        x = 1 / x;
        N = -N;
    }

    double result = 1.0;
    while (N > 0) {
        if (N % 2 == 1) {
            result *= x;
        }
        x *= x;
        N /= 2;
    }

    return result;
}

int main() {
    cout << powxnFastExponentiation(2.0, 10) << endl;  // 1024.0
    return 0;
}
