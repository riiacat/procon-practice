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

class Node
{
public:
    int key;
    Node *right, *left, *parent;
    Node(){};
};

Node *root, *NIL;

void insert(int k)
{
    Node *y = NIL;
    Node *x = root;
    Node *z;

    z = new Node();
    z->key = k;
    z->left = NIL;
    z->right = NIL;

    while (x != NIL)
    {
        y = x;
        if (z->key < x->key)
        {
            x = x->left;
        }
        else
        {
            x = x->right;
        }
    }

    z->parent = y;
    if (y == NIL)
    {
        root = z;
    }
    else
    {
        if (z->key < y->key)
        {
            y->left = z;
        }
        else
        {
            y->right = z;
        }
    }
}

void inorder(Node *u)
{
    if (u == NIL)
        return;

    inorder(u->left);
    cout << u->key << ' ';
    inorder(u->right);
}

void preorder(Node *u)
{
    if (u == NIL)
        return;

    cout << u->key << ' ';
    preorder(u->left);
    preorder(u->right);
}

Node *find(Node *u, int k)
{
    while (u != NIL && k != u->key)
    {
        if (k < u->key)
            u = u->left;
        else
            u = u->right;
    }
    return u;
}

int main()
{
    int n, i, x;
    string com;

    cin >> n;
    REP(i, n)
    {
        cin >> com;
        if (com == "insert")
        {
            cin >> x;
            insert(x);
        }
        else if (com == "print")
        {
            inorder(root);
            cout << endl;
            preorder(root);
            cout << endl;
        }
        else if (com == "find")
        {
            cin >> x;
            Node *t = find(root, x);
            if (t != NIL)
                cout << "yes" << endl;
            else
                cout << "no" << endl;
        }
    }

    return 0;
}