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

uint64_t popcnt(uint64_t n)
{
    uint64_t c = 0;
    c = (n & 0x5555555555555555) + ((n >> 1) & 0x5555555555555555);
    c = (c & 0x3333333333333333) + ((c >> 2) & 0x3333333333333333);
    c = (c & 0x0f0f0f0f0f0f0f0f) + ((c >> 4) & 0x0f0f0f0f0f0f0f0f);
    c = (c & 0x00ff00ff00ff00ff) + ((c >> 8) & 0x00ff00ff00ff00ff);
    c = (c & 0x0000ffff0000ffff) + ((c >> 16) & 0x0000ffff0000ffff);
    c = (c & 0x00000000ffffffff) + ((c >> 32) & 0x00000000ffffffff);
    return (c);
}

// const int LimN = 2 * (1e5);

// int N;
// int H[LimN];
// int A[LimN];

// LL dp[LimN + 1];
// LL dp_old[LimN + 1];

// int max_h;

// int main()
// {
//     ios::sync_with_stdio(false);

//     cin >> N;
//     REP(i, N)
//     {
//         cin >> H[i];
//         max_h = max(max_h, H[i]);
//     }
//     REP(i, N)
//     {
//         cin >> A[i];
//     }

//     // REP(i, N + 1)
//     // {
//     //     REP(j, N + 1)
//     //     {
//     //         dp[i][j] = -1;
//     //     }
//     // }

//     REP(i, N + 1)
//     {
//         dp[i] = 0;
//     }
//     // REP(j, N + 1)
//     // {
//     //     dp[0][j] = 0;
//     // }

//     FOR(n, 1, N + 1)
//     {
//         int nidx = n - 1;
//         memcpy(dp_old, dp, sizeof(dp));
//         FOR(h, H[nidx], (max_h + 1) + 1)
//         {
//             dp[h] = max(
//                 dp_old[H[nidx] - 1] + A[nidx], dp_old[h]);
//             // auto c = ", ";
//             // cout << h << c << n << c << dp[h][n] << endl;
//         }
//     }

//     LL ans = 0;
//     FOR(h, 0, (max_h + 1) + 1)
//     {
//         ans = max(ans, dp[h]);
//     }

//     cout << ans << endl;
// }
#define def 0

template <class V, int NV>
struct SegTree
{ //[l,r)
    V comp(V &l, V &r) { return max(l, r); };

    vector<V> val;
    SegTree() { val = vector<V>(NV * 2, def); }
    V get(int x, int y, int l = 0, int r = NV, int k = 1)
    {
        if (r <= x || y <= l)
            return def;
        if (x <= l && r <= y)
            return val[k];
        auto a = get(x, y, l, (l + r) / 2, k * 2);
        auto b = get(x, y, (l + r) / 2, r, k * 2 + 1);
        return comp(a, b);
    }
    void update(int i, V v)
    {
        i += NV;
        val[i] = v;
        while (i > 1)
            i >>= 1, val[i] = comp(val[i * 2], val[i * 2 + 1]);
    }
    void add(int i, V v) { update(i, val[i + NV] + v); }
    V operator[](int x) { return get(x, x + 1); }
};

int N, H[201010], A[201010];
SegTree<LL, 1 << 18> dp;
//---------------------------------------------------------------------------------------------------
int main()
{
    cin >> N;
    FOR(i, 0, N)
    cin >> H[i];
    FOR(i, 0, N)
    cin >> A[i];

    FOR(i, 0, N)
    {
        LL opt = dp.get(0, H[i]) + A[i];
        dp.update(H[i], max(dp[H[i]], opt));
    }

    cout << dp.get(0, 1 << 18) << endl;
}