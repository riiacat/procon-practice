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

const int LimitN = 1e5;
const int mod = 1e9 + 7;
VI edges[LimitN];
int N;

int parents[LimitN];
VI children[LimitN];

LL dp[LimitN][2]; //0: 白, 1:黒

void dfs(int v, int p)
{
    for (auto &e : edges[v])
    {
        if (e == p)
            continue;
        parents[e] = v;
        children[v].push_back(e);
        // cout << "children " << v << " -> " << e << endl;
        dfs(e, v);
    }
}

LL solve(int v, bool isBlack)
{
    // cout << "solve: " << v << endl;
    if (dp[v][isBlack ? 1 : 0] > 0)
    {
        return dp[v][isBlack ? 1 : 0];
    }

    LL ans = 1;
    for (auto &e : children[v])
    {
        LL mid = 0;
        mid += solve(e, false);
        mid %= mod;
        if (!isBlack)
        {
            mid += solve(e, true);
            mid %= mod;
        }

        ans *= mid;
        ans %= mod;
    }

    dp[v][isBlack ? 1 : 0] = ans;
    return ans;
}

int main()
{
    ios::sync_with_stdio(false);

    cin >> N;

    REP(i, N - 1)
    {
        int x, y;
        cin >> x >> y;
        x--;
        y--;
        edges[x].push_back(y);
        edges[y].push_back(x);
    }

    REP(i, N)
    {
        dp[i][0] = -1;
        dp[i][1] = -1;
    }

    //木を構築
    dfs(0, -1);

    //false: 白, true: 黒

    cout << (solve(0, true) + solve(0, false)) % mod << endl;
}