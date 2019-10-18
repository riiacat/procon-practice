//木のDFS実装

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

//clear memory
#define CLR(a) memset((a), 0, sizeof(a))

//debug
// #define dump(x) cerr << #x << " = " << (x) << endl;
// #define debug(x) cerr << #x << " = " << (x) << " (L" << __LINE__ << ")" \
//                       << " " << __FILE__ << endl;

using node = pair<LL, LL>; // vert index, cost

using ll = long long;
const int limit = 100010;
using edge = struct
{
    int to;
    ll cost;
};

vector<int> tree[limit];
// ll depth[limit];
ll max_depth[limit];

int dfs(int from, int v, ll d)
{
    // cout << from << ", " << v << ", " << d << endl;
    if (max_depth[v] > 0)
    {
        max_depth[from] = max(d + max_depth[v], max_depth[from]);
        return d + max_depth[v];
    }

    int ans = -1;
    for (auto &to : tree[v])
    {
        int dfs_res = dfs(from, to, d + 1);
        max_depth[v] = max(dfs_res - d, max_depth[v]);

        ans = max(dfs_res, ans);
    }

    max_depth[from] = ans < 0 ? d : ans;
    return max_depth[from];
}

int main(void)
{
    ios::sync_with_stdio(false);
    int n, m;
    cin >> n >> m;

    for (int i = 0; i < m; ++i)
    {
        int x, y;
        cin >> x >> y;
        x--, y--;
        tree[x].push_back(y);
    }

    int ans = -1;
    FOR(i, 0, n)
    {
        int res = dfs(i, i, 0);
        // cout << "dfs( " << i << " ) =" << res << endl;
        ans = max(res, ans);
    }

    cout << ans << endl;
    // for (int i = 0; i < q; ++i)
    // {
    //     int x, y;
    //     cin >> x >> y;
    //     x--, y--;
    //     cout << depth[x] + depth[y] << endl;
    // }
}
