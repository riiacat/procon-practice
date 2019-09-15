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

class LCS_Solver
{
    vector<VL> vl;
    string &s1;
    string &s2;

public:
    LCS_Solver(size_t s, string &s1, string &s2)
        : vl(s, VL(s, -1)), s1(s1), s2(s2)
    {
        vl[0][0] = 0;
        REP(i, s)
        {
            vl[0][i] = 0;
            vl[i][0] = 0;
        }
    }

    LL solve_memoize(int s1n, int s2m);
    LL solve_dp();
};

LL LCS_Solver::solve_memoize(int s1n, int s2m)
{
    // cout << s1n << ", " << s2m << endl;
    if (vl[s1n][s2m] >= 0)
    {
        return vl[s1n][s2m];
    }
    else
    {
        LL ans;
        if (s1[s1n - 1] == s2[s2m - 1])
        {
            ans =
                solve_memoize(s1n - 1, s2m - 1) + 1;
            vl[s1n][s2m] = ans;
            return ans;
        }
        else
        {
            ans = max(
                solve_memoize(s1n - 1, s2m),
                solve_memoize(s1n, s2m - 1));
            vl[s1n][s2m] = ans;
            return ans;
        }
    }
}

LL LCS_Solver::solve_dp()
{
    int n = s1.size();
    int m = s2.size();

    FOR(i, 0, n)
    {
        int I = i + 1;
        FOR(j, 0, n)
        {
            int J = j + 1;
            if (s1[i] == s2[j])
            {
                vl[I][J] =
                    vl[I - 1][J - 1] + 1;
            }
            else
            {
                vl[I][J] =
                    max(vl[I - 1][J],
                        vl[I][J - 1]);
            }
        }
    }

    return vl[n][m];
}

const size_t MAX_SIZE = 1001;

int main()
{
    int n;
    cin >> n;

    LL ans;
    REP(_, n)
    {
        string s1, s2;
        cin >> s1;
        cin >> s2;
        // cout << s1.size() << " " << s2.size() << endl;

        ans = LCS_Solver(MAX_SIZE, s1, s2)
                  .solve_dp();
        cout << ans << endl;
    }
}