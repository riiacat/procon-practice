#include <iostream>

//include
//------------------------------------------
#include <vector>
#include <list>
#include <map>
#include <set>
#include <deque>
#include <queue>
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

int main()
{
    long N, M;
    cin >> N >> M;

    priority_queue<long> q;

    REP(i, N)
    {
        long a;
        cin >> a;
        q.push(a);
    }

    REP(i, M)
    {
        long a = q.top();
        q.pop();

        q.push(a / 2);
    }

    long long sum = 0;
    REP(i, N)
    {

        sum += q.top();
        q.pop();
    }

    cout << sum << endl;
}