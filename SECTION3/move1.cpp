#include <string>
using namespace std;

// C++ 코드
int main()
{
	string s1 = "to be or not to be";	
	string s2 = s1;

	string s3 = "to be or not to be";
	string s4 = move(s3);
}
