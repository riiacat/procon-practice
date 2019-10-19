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

const int LimitN=400;
LL A[LimitN];
LL sums[LimitN+1];
LL dp[LimitN][LimitN];


int main()
{
    ios::sync_with_stdio(false);

    int N;
    cin >> N;
    sums[0] = 0;
    REP(i, N){
        LL a;
        cin >> a;
        A[i] = a;
        sums[i+1] = sums[i] + a; 
        dp[i][i] = 0;
    }


    for(int len=1; len <= N ; len++){
        for(int left=0; left+len < N; left++){
            int right = left+len;
            LL ans = __LONG_LONG_MAX__ / 10;
            for(int sep = left+1; sep <= right; sep++){
                LL mid = (sums[sep] - sums[left]) + (sums[right+1] - sums[sep]) +dp[left][sep-1] + dp[sep][right];
                ans = min(
                    mid
                    , ans
                );
                // cout << left << ", " << sep << ", " << right << ", " << mid << endl;
            }
            dp[left][right] = ans;
            // cout<< left << ", " << right << ", " << dp[left][right] <<endl;
        }
    }
    
    cout << dp[0][N-1] << endl;
}