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

class FibSolver
{
private:
    vector<LL> &ans_holder;

public:
    FibSolver(vector<LL> &vec) : ans_holder(vec)
    {
        ans_holder[0] = 1; // f(1)
        ans_holder[1] = 1; // f(2)
    }

    long long solve_reccursive(int n) const;

    long long solve_memoize(int n);

    long long solve_dp(int n) const;

    long long solve_general(int n) const;
};

LL FibSolver::solve_reccursive(int n) const
{
    if (n == 0 || n == 1)
        return 1;
    else
        return solve_reccursive(n - 1) + solve_reccursive(n - 2);
}

LL FibSolver::solve_memoize(int n)
{
    if (n > ans_holder.size())
    {
        cout << n << " >= " << ans_holder.size() << endl;
        throw exception();
    }

    int idx = n;

    if (ans_holder[idx] > 0)
    {
        return ans_holder[idx];
    }
    else
    {
        LL ans = solve_memoize(n - 1) + solve_memoize(n - 2);
        ans_holder[idx] = ans;
        return ans;
    }
}

LL FibSolver::solve_dp(int n) const
{
    LL former1 = 1;
    LL former2 = 1;
    LL ans;

    FOR(i, 2, n + 1)
    {
        ans = former1 + former2;
        former1 = former2;
        former2 = ans;
    }

    return ans;
}

const int MAX_ELEMENT = 100000;

int main()
{
    vector<LL> holder(MAX_ELEMENT, -1);

    FibSolver solver(holder);

    int n;
    cin >> n;
    LL ans;
    try
    {
        ans = solver.solve_dp(n);
    }
    catch (exception e)
    {
        cout << "Exception!!!!" << endl;
        ans = -1;
    }

    cout << ans << endl;
}