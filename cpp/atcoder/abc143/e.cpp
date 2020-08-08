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

int N, M, L;

const int LimN = 300;
vector<PII> es[LimN];
int C[LimN][LimN];

int solve(int s, int t)
{
    return 0;
}

typedef pair<int, int> P; // first: 最短距離, second: 頂点番号

LL d[LimN];
int from[LimN];

void dijkstra(int s)
{
    d[s] = 0; // 始点sへの最短距離は0
    from[s] = -1;

    REP(i, N)
    {
        d[i] = __LONG_LONG_MAX__ / 10;
        from[i] = -1;
    }
    d[s] = 0;

    priority_queue<P, vector<P>, greater<P>> que; // 距離が小さい順に取り出せるようgreater<P>を指定
    que.push(P(0, s));

    while (!que.empty())
    {
        P p = que.top();
        que.pop();
        int v = p.second; // 更新した頂点の中で距離が最小の頂点v
        if (d[v] < p.first)
        {
            continue;
        }
        for (auto e : es[v])
        { // 頂点vから出る辺eを走査
            LL new_d = d[v] + e.second;
            if (d[e.first] > new_d && e.second <= L)
            {
                d[e.first] = d[v] + e.second;
                from[e.first] = v;
                que.push(P(d[e.first], e.first));
            }
        }
    }
}

int main()
{
    ios::sync_with_stdio(false);

    cin >> N >> M >> L;

    REP(i, M)
    {
        int a, b, c;
        cin >> a >> b >> c;
        a--;
        b--;
        es[a].push_back(make_pair(b, c));
        es[b].push_back(make_pair(a, c));
        C[a][b] = c;
        C[b][a] = c;
    }

    int Q;
    cin >> Q;
    REP(i, Q)
    {
        int s, t;
        cin >> s >> t;
        s--;
        t--;

        dijkstra(s);
        // REP(i, N)
        // {
        //     cout << "d[ " << i << "] = " << d[i] << endl;
        // }

        if (d[t] == __LONG_LONG_MAX__ / 10)
        {
            cout << -1 << endl;
        }
        else
        {
            vector<int> path;
            path.push_back(t);
            int tt = t;
            while (true)
            {
                int f = from[tt];
                // cout << tt << ", " << f << endl;
                if (f == -1)
                {
                    break;
                }
                path.push_back(f);
                tt = f;
            }

            int ans = 0;
            LL use = 0;
            FOR(i, 0, path.size() - 1)
            {
                int c = C[path[i]][path[i + 1]];
                if (use + c > L)
                {
                    ans++;
                    use = 0;
                }
                use += c;
            }

            cout << ans << endl;
        }
    }
}