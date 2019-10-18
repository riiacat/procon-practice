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

int N;
LL dp1[3000][3000];
LL dp2[3000][3000];
int A[3000];

int main()
{
    ios::sync_with_stdio(false);

    cin >> N;
    REP(i, N)
    {
        int a;
        cin >> a;
        A[i] = a;
        dp1[i][i] = a;
        dp2[i][i] = a;
    }

    REP(i, N - 1)
    {
        dp1[i][i + 1] = max(A[i] - A[i + 1], A[i + 1] - A[i]);
        dp2[i][i + 1] = min(A[i] - A[i + 1], A[i + 1] - A[i]);
    }

    FOR(len, 3, N + 1)
    {
        for (int l = 0; l <= N - len; l = l + 1)
        {
            int r = l + len - 1;
            // cout << l << ", " << r << endl;
            dp1[l][r] = max(
                A[l] + dp2[l + 1][r], A[r] + dp2[l][r - 1]);
            dp2[l][r] = min(
                dp1[l + 1][r] - A[l], dp1[l][r - 1] - A[r]);
        }
    }

    cout << (max(A[0] + dp2[1][N - 1], A[N - 1] + dp2[0][N - 2])) << endl;
}