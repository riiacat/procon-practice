//ダイクストラ実装

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

enum Color
{
    Black,
    White,
    Gray
};

int main()
{
    int N;
    cin >> N;

    vector<vector<node>> nodes(N, vector<node>());
    REP(i, N - 1)
    {
        int a, b;
        LL c;
        cin >> a >> b >> c;
        a = a - 1;
        b = b - 1;
        nodes[a].push_back(make_pair(b, c));
        nodes[b].push_back(make_pair(a, c));
    }

    vector<LL> lowest_costs(N, __LONG_LONG_MAX__ - 1);
    vector<Color> colors(N, Color::White);

    int Q, K;
    cin >> Q >> K;

    K = K - 1;

    lowest_costs[K] = 0;

    auto cmp = [](node a, node b) { return a.second > b.second; };
    priority_queue<node, vector<node>, decltype(cmp)> q(cmp);

    int idx = 0;

    EACH(i, lowest_costs)
    {
        q.push(make_pair(idx, *i));
        idx++;
    }

    while (!q.empty())
    {
        auto u = q.top();
        q.pop();

        // cout << u.first << ", " << u.second << endl;

        colors[u.first] = Color::Black;

        if (lowest_costs[u.first] < u.second)
            continue;

        EACH(nde, nodes[u.first])
        {
            auto to = nde->first;
            auto cost_to = nde->second;
            if (colors[to] != Color::Black)
            {
                auto new_cost = lowest_costs[u.first] + cost_to;

                if (new_cost < lowest_costs[to])
                {
                    lowest_costs[to] = new_cost;
                    colors[to] = Color::Gray;
                    q.push(make_pair(to, lowest_costs[to]));
                }
            }
        }
    }

    REP(i, Q)
    {
        int x, y;
        cin >> x >> y;
        x = x - 1;
        y = y - 1;

        cout << lowest_costs[x] + lowest_costs[y] << endl;
    }
}