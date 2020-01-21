#include <iostream>
#include <math.h>
using namespace std;

int main() {
    string numLine;
    getline(cin, numLine);
    int num = stoi(numLine);
    int numDays = ceil(log2(num)) + 1;
    cout << numDays << endl;
    return 0;
}
