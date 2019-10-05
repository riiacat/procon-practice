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

enum Behav
{
    SEA,
    Mt,
    HOME
};

int N;
VI A, B, C;

int main()
{
    cin >> N;
    REP(i, N)
    {
        int a, b, c;
        cin >> a >> b >> c;
        A.push_back(a);
        B.push_back(b);
        C.push_back(c);
    }

    VL max_seas(N, -1);
    VL max_mts(N, -1);
    VL max_homes(N, -1);

    max_seas[0] = A[0];
    max_mts[0] = B[0];
    max_homes[0] = C[0];

    FOR(day, 1, N)
    {
        max_seas[day] = max(
            {max_mts[day - 1] + A[day],
             max_homes[day - 1] + A[day]});
        max_mts[day] = max(
            {max_seas[day - 1] + B[day],
             max_homes[day - 1] + B[day]});
        max_homes[day] = max(
            {max_mts[day - 1] + C[day],
             max_seas[day - 1] + C[day]});
    }
    LL ans = max({
        max_seas[N - 1],
        max_mts[N - 1],
        max_homes[N - 1],
    });
    cout << ans << endl;
}