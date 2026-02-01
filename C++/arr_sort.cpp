#include <iostream>
#include <algorithm>
using namespace std;

auto int_from( istream& in ) -> int { int x; in >> x; return x; }

int main(){
    cout << "Please enter how many elements the array you want to have: ";
    int n = int_from( cin );
    int* arr = new int [n];

    for(int i = 0; i < n; i++){
        cout << "Please enter the #" << i+1 << " element of the array: "; 
        arr[i] = int_from( cin );
    }

    sort( arr, arr+n);

    for(int i = 0; i < n; i++){
        cout << arr[i] << "\n";
    }

    delete[] arr;

    return 0;
}
