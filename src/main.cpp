#include <iostream>
#include <string>

using namespace std;

class Person
{
public:
    string &name;
    // Person(const Person& rhs);
    Person(string &str) : name(str) {}
};

int main()
{
    string s1 = "alice", s2 = "bob";
    Person p1(s1), p2(s2);
    cout << "p1" << endl;
    // p1 = p2;
    return 0;
}
