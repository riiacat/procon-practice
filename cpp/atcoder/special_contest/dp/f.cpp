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

int main()
{
    string s, t;
    cin >> s >> t;

    int n = s.length();
    int m = t.length();

    vector<vector<int>> dp(n + 1, vector<int>(m + 1, 0));

    FOR(i, 1, n + 1)
    {
        FOR(j, 1, m + 1)
        {
            int c1, c2, c3;

            c1 = dp[i - 1][j];
            c2 = dp[i][j - 1];

            if (c1 > c2)
            {
                if (s[i - 1] == t[j - 1])
                {
                    c3 = dp[i - 1][j - 1];
                    if (c3 + 1 > c1)
                    {
                        dp[i][j] = c3 + 1;
                    }
                    else
                    {
                        dp[i][j] = c1;
                    }
                }
                else
                {
                    dp[i][j] = c1;
                }
            }
            else
            {
                if (s[i - 1] == t[j - 1])
                {
                    c3 = dp[i - 1][j - 1];
                    if (c3 + 1 > c2)
                    {
                        dp[i][j] = c3 + 1;
                    }
                    else
                    {
                        dp[i][j] = c2;
                    }
                }
                else
                {
                    dp[i][j] = c2;
                }
            }
        }
    }

    string ans = "";
    ans.reserve(3000);
    while (n > 0 && m > 0)
    {
        if (dp[n][m] == dp[n - 1][m])
        {
            n--;
        }
        else if (dp[n][m] == dp[n][m - 1])
        {
            m--;
        }
        else if (dp[n][m] == dp[n - 1][m - 1] + 1)
        {
            ans += s[n - 1];
            n--;
            m--;
        }
    }

    reverse(ans.begin(), ans.end());
    cout << ans << endl;
}