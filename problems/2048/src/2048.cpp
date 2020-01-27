#include <iostream>
#include <vector>
using namespace std;


int main() {
    int board[4][4];
    string lines[4];
    for (int i = 0; i < 4; i++) {
        string line;
        getline(cin, line);
        lines[i] = line;
        /* cout << line << endl; */
    }

    string directionLine;
    getline(cin, directionLine);
    /* cout << directionLine << endl; */
    int direction = stoi(directionLine, nullptr, 10);

    for (int i = 0; i < 4; i++) {
        string line = lines[i];
        int lastSpaceIndex = 0;
        int insertI;
        int insertJ;
        if (direction == 0) { //left
            insertI = i;
        } else if (direction == 2) { //right
            insertI = i;
        } else if (direction == 1) { // up
            insertJ = i;
        } else if (direction == 3) { // down
            insertJ = 3 - i;
        }
        for (int j = 0; j < 4; j++) {
            if (direction == 0) {
                insertJ = j;
            } else if (direction == 2) { //right
                insertJ = 3 - j;
            } else if (direction == 1) { // up
                insertI = 3 - j;
            } else if (direction == 3) { // down
                insertI = j;
            }
            int spaceIndex;
            if (j == 3) {
                spaceIndex = line.size();
            } else {
                spaceIndex = line.find(" ", lastSpaceIndex + 1);
                /* int spaceIndex = static_cast<int>(line.find(" ", lastSpaceIndex + 1)); */
            }
            int num = stoi(line.substr(lastSpaceIndex, spaceIndex - lastSpaceIndex), nullptr, 10);
            /* cout << line << endl; */
            /* cout << i << "   " << j << endl; */
            /* cout << spaceIndex << "   " << lastSpaceIndex << endl; */
            /* cout << "num: " << num << endl; */
            board[insertI][insertJ] = num;
            lastSpaceIndex = spaceIndex;
        }
    }

    
   vector<int> lst[4];
   for (int i = 0; i < 4; i++) {
       /* lst[i] = vector<int>; */
       int prevNum = 0;
       for (int j = 0; j < 4; j++) {
           int num = board[i][j];
           /* cout << num <<endl; */
           if (num != 0) {
               if (prevNum == num) {
                   lst[i].push_back(num * 2);
                   prevNum = 0;
               } else if (prevNum != 0) {
                   lst[i].push_back(prevNum),
                   prevNum = num;
               } else {
                   prevNum = num;
               }
           } 
           /* else if (prevNum != 0) { */
           /*     lst[i].push_back(prevNum); */
           /*     prevNum = num; */
           /* } */
       }
       if (prevNum != 0) {
           lst[i].push_back(prevNum);
       }
   }


   int outboard[4][4] = {{0}};
   for (int i = 0; i < 4; i++) {
       int j = 0;
       for (vector<int>::iterator it = lst[i].begin(); it != lst[i].end(); ++it) {
            /* cout << *it << endl; */
            if (direction == 0) {
                outboard[i][j] = *it;
            } else if (direction == 2) {
                outboard[i][3 - j] = *it;
            } else if ( direction == 1 ) {
                outboard[j][3 - i] = *it;
            } else if (direction == 3) {
                outboard[3 - j][i] = *it;
            }

            j++;
        }
    }
    
    for (int i = 0; i < 4; i++) {
        for (int j = 0; j < 4; j++) {
            cout << outboard[i][j];
            if (j != 3) {
                cout << " ";
            }
        }
        cout << endl;
    }

    return 0;
}
