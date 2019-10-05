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
const int maxW = 1e5;

long long max_values[maxN + 1][maxW + 1];

int main()
{
    cin >> N >> W;
    VI weights, V;
    REP(i, N)
    {
        int w, v;
        cin >> w >> v;
        weights.push_back(w);
        V.push_back(v);
    }

    FOR(i, 0, N + 1)
    {
        FOR(j, 0, W + 1)
        {
            max_values[i][j] = -INFI;
        }
    }

    FOR(i, 0, W + 1)
    {
        max_values[0][i] = 0;
    }

    FOR(i, 0, N + 1)
    {
        max_values[i][0] = 0;
    }

    FOR(items, 1, N + 1)
    {
        FOR(weight_lim, 1, W + 1)
        {
            int reduced_weight = weight_lim - weights[items - 1];
            if (reduced_weight >= 0)
            {
                max_values[items][weight_lim] =
                    max({max_values[items - 1][weight_lim], max_values[items - 1][reduced_weight] + V[items - 1]});
            }
            else
            {
                max_values[items][weight_lim] =
                    max_values[items - 1][weight_lim];
            }
        }
    }
    cout << max_values[N][W] << endl;
}