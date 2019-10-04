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

//clear memory
#define CLR(a) memset((a), 0, sizeof(a))

//debug
// #define dump(x) cerr << #x << " = " << (x) << endl;
// #define debug(x) cerr << #x << " = " << (x) << " (L" << __LINE__ << ")" \
//                       << " " << __FILE__ << endl;

int N, M;
vector<bitset<12>> keys;
vector<LL> costs;

LL min_cost(bitset<12> bits, int first)
{
    // cout << bits << endl;
    // cout << bits << endl;
    LL tmp_min = 1000000001;
    FOR(i, first, M)
    {
        // cout << "idx: " << i << endl;
        // cout << "op: " << ((~bits) | keys[i]) << endl;
        if (((~bits) | keys[i]).all())
        {
            tmp_min = min(tmp_min, costs[i]);
        }
        else if ((bits & keys[i]).any())
        {
            // cout << "next: " << (bits & (~keys[i])) << endl;

            tmp_min = min(tmp_min, costs[i] + min_cost(bits & (~keys[i]), i + 1));
        }
    }

    // cout << tmp_min << endl;
    return tmp_min;
}

int main()
{
    cin >> N >> M;

    REP(i, M)
    {
        int a, b;
        cin >> a >> b;
        keys.push_back(bitset<12>());
        costs.push_back(a);
        REP(j, b)
        {
            int c;
            cin >> c;
            c--;
            keys[i][c] = true;
        }

        for (int k = N; k < 12; k++)
        {
            keys[i][k] = true;
        }

        // cout << keys[i] << endl;
    }

    auto ans = min_cost(bitset<12>("111111111111"), 0);
    cout << (ans > 1000000000 ? -1 : ans) << endl;

    // bitset<12>
    //     bits;
    // cout << bits << endl;
}