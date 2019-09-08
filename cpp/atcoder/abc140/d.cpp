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

int main()
{
    int n;
    cin >> n;

    int k;
    cin >> k;

    string S;
    cin >> S;

    vector<bool> LRs = vector<bool>(n - 1, false);
    vector<bool> RLs = vector<bool>(n - 1, false);

    int ans = 0;
    FOR(i, 0, n - 1)
    {
        if (S[i] == 'L' && S[i + 1] == 'R')
        {
            LRs[i] = true;
        }

        if (S[i] == 'R' && S[i + 1] == 'L')
        {
            RLs[i] = true;
        }

        if (S[i] == 'L' && S[i + 1] == 'L')
        {
            ans += 1;
        }

        if (S[i] == 'R' && S[i + 1] == 'R')
        {
            ans += 1;
        }
    }

    int count2 = 0;
    int count1 = 0;
    FOR(i, 0, n - 1)
    {
        if (LRs[i])
        {
            bool not_found = true;
            FOR(j, i, n - 1)
            {
                if (RLs[j])
                {
                    // ans += 2;
                    // RLs[j] = false;
                    not_found = false;
                    count2 += 1;
                    break;
                }
            }
            if (not_found)
                count1 += 1;
            break;
        }

        if (RLs[i])
        {
            bool not_found = true;
            FOR(j, i, n - 1)
            {
                if (LRs[j])
                {
                    // LRs[j] = false;
                    count2 += 1;
                    not_found = false;
                    break;
                }
            }
            if (not_found)
                count1 += 1;
        }
    }

    if (count2 < k)
    {
        ans += count2 * 2;
        ans += k - count2 < count1 ? k - count2 : count1;
    }
    else
    {
        ans += k * 2;
    }

    cout << ans << endl;
}
