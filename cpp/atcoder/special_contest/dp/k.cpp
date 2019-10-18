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

const int limit = 1e5 + 1;
int dp[limit];
int A[100];
set<int> A_set;

int N, K;
int min_a = 1e8;

// bool solve(int k)
// {
//     if (dp[k] > 0)
//     {
//         return true;
//     }
//     else if (dp[k] < 0)
//     {
//         return false;
//     }

//     if (A_set.find(k) != A_set.end())
//     {
//         return true;
//     }
//     else if (k < min_a)
//     {
//         return false;
//     }
//     else
//     {
//         bool isWin = false;
//         REP(i, N)
//         {
//             isWin |= !solve(k - A[i]);
//         }
//         dp[k] = isWin ? 1 : -1;
//         return isWin;
//     }
// }

int main()
{
    ios::sync_with_stdio(false);

    cin >> N >> K;

    REP(i, N)
    {
        int a;
        cin >> a;
        A[i] = a;
        min_a = min(min_a, a);
        A_set.insert(a);
    }

    REP(i, K + 1)
    {
        dp[N] = 0;
    }

    stack<int> s;
    s.push(K);

    while (!s.empty())
    {
        int k = s.top();
        // cout << k << endl;

        if (dp[k] != 0)
        {
            s.pop();
            continue;
        }

        if (A_set.find(k) != A_set.end())
        {
            dp[k] = 1;
            s.pop();
            continue;
        }
        else if (k < min_a)
        {
            dp[k] = -1;
            s.pop();
            continue;
        }
        else
        {
            bool isWin = false;
            bool isNotCalced = false;
            REP(i, N)
            {
                if (dp[k - A[i]] == 0)
                {
                    REP(j, N)
                    {
                        if (dp[k - A[j]] == 0)
                            s.push(k - A[j]);
                    }
                    isNotCalced = true;
                }
                isWin |= !(dp[k - A[i]] > 0 ? true : false);
            }

            if (!isNotCalced)
            {
                dp[k] = isWin ? 1 : -1;
                s.pop();
                continue;
            }
        }
    }

    cout << (dp[K] > 0 ? "First" : "Second") << endl;
}