#include <iostream>

//include
//------------------------------------------
#include <vector>
#include <list>
#include <map>
#include <set>
#include <deque>
#include <stack>
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
#define EACH(i, c) for (typeof((c).begin()) i = (c).begin(); i != (c).end(); ++i)
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

class MCM_Solver
{
private:
    vector<tuple<LL, LL>> dims;
    vector<VL> dp;

public:
    LL solve_dp();
    MCM_Solver(vector<tuple<LL, LL>> dims, size_t n) : dims(dims), dp(n, VL(n, LONG_MAX))
    {
        REP(i, n)
        {
            dp[i][i] = 0;
        }
    }
};

LL MCM_Solver::solve_dp()
{
    int n = dp.size();
    FOR(l, 2, n + 1)
    {
        FOR(i, 0, (n - l + 1))
        {
            int j = i + l - 1;
            LL ans = LONG_MAX;
            FOR(k, i, j)
            {
                ans = min(
                    ans,
                    dp[i][k] + dp[k + 1][j] + get<0>(dims[i]) * get<0>(dims[k + 1]) * get<1>(dims[j]));
            }
            dp[i][j] = ans;
            // cout << i << ", " << j << ", " << dp[i][j] << endl;
        }
    }

    return dp[0][n - 1];
}

int main()
{
    // cout << LONG_MAX << endl;

    int n;
    cin >> n;
    vector<tuple<LL, LL>> dims;
    REP(i, n)
    {
        int nn, m;
        cin >> nn >> m;

        dims.push_back(make_tuple(nn, m));
    }

    LL ans = MCM_Solver(dims, n).solve_dp();

    cout << ans << endl;
}