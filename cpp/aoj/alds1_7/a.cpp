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

VI parents;
vector<VI> children;

int main()
{
    int n;
    cin >> n;

    parents = VI(n, -1);
    children = vector<VI>(n, VI());

    REP(i, n)
    {
        int id, k;
        cin >> id >> k;

        REP(child_i, k)
        {
            int c;
            cin >> c;

            children[i].push_back(c);
        }

        EACH(child_iter, children[i])
        {
            parents[*child_iter] = id;
        }
    }

    REP(i, n)
    {
        //calc depth
        int depth = 0;
        int traverse_idx = i;
        while (true)
        {
            if (parents[traverse_idx] == -1)
            {
                break;
            }
            else
            {
                traverse_idx = parents[traverse_idx];
                depth++;
            }
        }

        string t_type;
        if (depth == 0)
        {
            t_type = "root";
        }
        else if (children[i].size() != 0)
        {
            t_type = "internal node";
        }
        else
        {
            t_type = "leaf";
        }

        //node
        cout << "node " << i << ": parent = " << parents[i] << ", depth = "
             << depth << ", " << t_type << ", ";

        //chldren
        cout << '[';
        EACH(iter, children[i])
        {
            cout << *iter;
            if (next(iter) != children[i].end())
            {
                cout << ", ";
            }
        }
        cout << ']' << endl;
    }
}