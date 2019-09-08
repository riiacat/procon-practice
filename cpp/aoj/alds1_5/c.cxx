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

int n;
using point = tuple<double, double>;

tuple<point, point, point> get_points(point p1, point p2)
{
    double x1 = get<0>(p1);
    double y1 = get<1>(p1);
    double x2 = get<0>(p2);
    double y2 = get<1>(p2);

    auto delta = make_tuple(x2 - x1, y2 - y1);
    double deltax = get<0>(delta);
    double deltay = get<1>(delta);

    auto s = make_tuple(x1 + deltax / 3.0, y1 + deltay / 3.0);
    auto t = make_tuple(x1 + deltax / 3.0 * 2.0, y1 + deltay / 3.0 * 2.0);

    double to_u_x = (deltax / 2.0 - deltay * sqrt(3.0) / 2.0) / 3.0;
    double to_u_y = (deltax * sqrt(3.0) / 2.0 + deltay / 2.0) / 3.0;

    auto u = make_tuple(x1 + get<0>(s) + to_u_x, y1 + get<1>(s) + to_u_y);

    return make_tuple(s, u, t);
}

void print_p(point p1)
{

    double x1 = get<0>(p1);
    double y1 = get<1>(p1);

    cout << setprecision(10) << x1 << ' ' << y1 << endl;
}

void print_point(point p1, point p2, int count)
{
    if (count == 0)
        return;

    auto next_points = get_points(p1, p2);
    auto s = get<0>(next_points);
    auto u = get<1>(next_points);
    auto t = get<2>(next_points);

    print_point(p1, s, count - 1);
    print_p(s);
    print_point(s, u, count - 1);
    print_p(u);
    print_point(u, t, count - 1);
    print_p(t);
    print_point(t, p2, count - 1);
}

int main()
{
    cin >> n;

    // tuple<int, int> a = make_tuple(1, 1);
    auto a = make_tuple(1.0, 1.0);

    print_p(make_tuple(0.0, 0.0));
    print_point(make_tuple(0.0, 0.0), make_tuple(100.0, 0.0), n);
    print_p(make_tuple(100.0, 0.0));
}
