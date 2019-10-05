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

int N, W;

const int maxN = 100;
const int maxV = 1e5;

long long min_weight[maxN + 1][maxV + 1];

int main()
{
    cin >> N >> W;
    VI weights, V;
    int total_v = 0;
    REP(i, N)
    {
        int w, v;
        cin >> w >> v;
        weights.push_back(w);
        V.push_back(v);
        total_v += v;
    }

    FOR(i, 0, N + 1)
    {
        FOR(j, 0, total_v + 1)
        {
            min_weight[i][j] = INFL;
        }
    }

    FOR(i, 0, N + 1)
    {
        min_weight[i][0] = 0;
    }

    FOR(itm, 1, N + 1)
    {
        FOR(val, 1, total_v + 1)
        {
            int needed_val = val - V[itm - 1];
            if (needed_val >= 0)
            {
                min_weight[itm][val] = min(
                    min_weight[itm - 1][val], min_weight[itm - 1][needed_val] + weights[itm - 1]);
            }
            else
            {
                min_weight[itm][val] =
                    min_weight[itm - 1][val];
            }

            // cout << itm << ", " << val << ", " << min_weight[itm][val] << endl;
        }
    }

    for (int v = total_v; v >= 0; v--)
    {
        if (min_weight[N][v] <= W)
        {
            cout << v << endl;
            break;
        }
    }
}