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

const int limit = 301;
double dp[limit][limit][limit];
int A[limit];

int N;

double solve(int i, int j, int k)
{
    // cout << i << ", " << j << ", " << k << endl;
    if (dp[i][j][k] > 0)
    {
        return dp[i][j][k];
    }
    else
    {
        double s1, s2, s3;
        s1 = i == 0 ? 0 : solve(i - 1, j, k);
        s2 = j == 0 ? 0 : solve(i + 1, j - 1, k);
        s3 = k == 0 ? 0 : solve(i, j + 1, k - 1);
        dp[i][j][k] = (N + i * s1 + j * s2 + k * s3) / ((i + j + k) * 1.0);
        return dp[i][j][k];
    }
}

int main()
{
    ios::sync_with_stdio(false);

    cin >> N;

    int I, J, K;
    I = J= K =0;
    REP(i, N)
    {
        int a;
        cin >> a;
        A[i] = a;
        switch (a)
        {
        case 1:
            I++;
            break;
        case 2:
            J++;
            break;
        case 3:
            K++;
            break;
        default:
            return 0;
            break;
        }
    }

    // cout << "aaa" << endl;

    // FOR(i, 0, N + 1)
    // {
    //     dp[i][0][0] = i;
    // }
    dp[0][0][0] = EPS;

    cout << std::setprecision(100) 
        << solve(I, J, K) << endl;
}