#include <iostream>

//include
//------------------------------------------
#include <vector>
#include <list>
#include <map>
#include <set>
#include <deque>
#include <stack>
#include <queue>
#include <bitset>
#include <tuple>
#include <algorithm>
#include <functional>
#include <numeric>
#include <utility>
#include <sstream>
#include <iostream>
#include <iomanip>
#include <cstdio>
#include <cmath>
#include <cstdlib>
#include <cctype>
#include <string>
#include <cstring>
#include <ctime>
#include <climits>

using namespace std;

//conversion
//------------------------------------------
inline int toInt(string s)
{
    int v;
    istringstream sin(s);
    sin >> v;
    return v;
}
template <class T>
inline string toString(T x)
{
    ostringstream sout;
    sout << x;
    return sout.str();
}

//math
//-------------------------------------------
template <class T>
inline T sqr(T x) { return x * x; }

//typedef
//------------------------------------------
typedef vector<int> VI;
typedef vector<VI> VVI;
typedef vector<long long> VL;
typedef vector<string> VS;
typedef pair<int, int> PII;
typedef long long LL;

//container util
//------------------------------------------
#define ALL(a) (a).begin(), (a).end()
#define RALL(a) (a).rbegin(), (a).rend()
#define PB push_back
#define MP make_pair
#define SZ(a) int((a).size())
#define EACH(i, c) for (auto i = (c).begin(); i != (c).end(); ++i)
#define EXIST(s, e) ((s).find(e) != (s).end())
#define SORT(c) sort((c).begin(), (c).end())

//repetition
//------------------------------------------
#define FOR(i, a, b) for (int i = (a); i < (b); ++i)
#define REP(i, n) FOR(i, 0, n)

//constant
//--------------------------------------------
const double EPS = 1e-10;
const double PI = acos(-1.0);
const long long INFL = __LONG_LONG_MAX__ / 10;
const int INFI = __INT_MAX__ / 10;

//clear memory
#define CLR(a) memset((a), 0, sizeof(a))

//debug
// #define dump(x) cerr << #x << " = " << (x) << endl;
// #define debug(x) cerr << #x << " = " << (x) << " (L" << __LINE__ << ")" \
//                       << " " << __FILE__ << endl;

double dp[3000][3000];

double p[2999];
int main()
{
    ios::sync_with_stdio(false);

    int N;
    cin >> N;

    REP(i, N)
    {
        double pp;
        cin >> pp;
        p[i] = pp;
    }

    dp[0][0] = 1;

    FOR(coin_N, 1, N + 1)
    {
        dp[coin_N][0] = dp[coin_N - 1][0] * (1 - p[coin_N - 1]);
    }

    FOR(coin_N, 1, N + 1)
    {
        dp[0][coin_N] = 0;
    }

    FOR(coin_N, 1, N + 1)
    {
        FOR(num1, 1, coin_N + 1)
        {
            dp[coin_N][num1] =
                dp[coin_N - 1][num1] * (1 - p[coin_N - 1]) + dp[coin_N - 1][num1 - 1] * p[coin_N - 1];
        }
    }

    double ans = 0;
    for (int n = N / 2 + 1; n <= N; n++)
    {
        ans += dp[N][n];
    }
    cout << std::setprecision(12) << ans << endl;
}