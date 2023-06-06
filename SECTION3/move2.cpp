#include <string>
using namespace std;

// C++ 코드
int main()
{
	string s1 = "to be or not to be";	
	string s2 = "practice make perfect";

	// swap using copy
	string t1 = s1;
	s1 = s2;
	s2 = t1;

	// swap using move
	string t2 = move(s1);
	s1 = move(s2);
	s2 = move(t2);
}
