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

int N, K;
int a[100];

const int limit_K = 1e5 + 1;
LL old_dp[limit_K];
LL dp[limit_K];

// int mod = 10e9 + 7;
int mod = 1000000007;
int main()
{
    ios::sync_with_stdio(false);
    cin >> N >> K;

    LL sums = 0;
    REP(i, N)
    {
        LL aa;
        cin >> aa;
        a[i] = aa;
        sums+= aa;
    }

    if(K > sums ){
        cout << 0 << endl;
        return 0;
    }

    REP(i, K + 1)
    {
        dp[i] = i <= a[0] ? 1 : 0;
    }

    FOR(n, 1, N)
    {
        memcpy(old_dp, dp, sizeof(dp));
        // FOR(aa, 1, a[n]+1){
        LL sums[limit_K];
        sums[0] = dp[0];
        // cout << "sums[ "<< 0<< "] " << sums[0] << endl;            
        FOR(k, 1, K+1){
            sums[k] = (sums[k-1] + dp[k])%mod;
            // cout << "sums[ "<< k<< "] " << sums[k] << endl;            
        }

        FOR(k,1, K+1){
            int left = max(-1, k-(a[n]+1));
            if(left==-1){
                dp[k] = sums[k];
            }else{
                dp[k] = (sums[k] - sums[left]) % mod;
            }
            // cout << "dp[ "<< k<< "] " << dp[k] << endl;
        }
        // }
    }

    cout << (dp[K] %mod) << endl;
}