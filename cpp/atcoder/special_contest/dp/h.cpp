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

const int mod = 1e9 + 7;
vector<string> maze;
LL dp[1000][1000];

int solve(int h, int w)
{

    if (dp[h][w] >= 0)
    {
        // cout << h << ", " << w << ", " << dp[h][w] << endl;
        return dp[h][w];
    }
    else
    {
        LL ans = 0;
        if (maze[h - 1][w] != '#')
        {
            ans += solve(h - 1, w);
            ans %= mod;
        }
        if (maze[h][w - 1] != '#')
        {
            ans += solve(h, w - 1);
            ans %= mod;
        }
        dp[h][w] = ans;

        // cout << h << ", " << w << ", " << dp[h][w] << endl;
        return ans;
    }
}

int main()
{
    ios::sync_with_stdio(false);

    int H, W;
    cin >> H >> W;

    REP(i, H)
    {
        string row;
        cin >> row;
        maze.push_back(row);
    }

    REP(i, H)
    {
        REP(j, W)
        {
            dp[i][j] = (maze[i][j] == '#') ? 0 : -1;
        }
    }

    bool isWall = false;
    REP(i, H)
    {
        if (maze[i][0] == '#')
        {
            isWall = true;
        }

        dp[i][0] = isWall ? 0 : 1;
    }

    isWall = false;
    REP(i, W)
    {
        if (maze[0][i] == '#')
        {
            isWall = true;
        }

        dp[0][i] = isWall ? 0 : 1;
    }

    cout << solve(H - 1, W - 1) << endl;
}