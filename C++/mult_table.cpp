#include <iostream>
using namespace std;

int main(){
    int rows;
    int cols;

    cout << "Please enter how many rows you'd like the multiplication table to have: ";
    cin >> rows;
    cout << "Please enter how many cols you'd like the multiplication table to have: ";
    cin >> cols;

    for(int i = 0; i < rows; i++){
        for(int j = 0; j < cols; j++){
            cout << (i+1) * (j+1) << ' ';
        }
        cout << '\n';
    }


    return 0;
}
