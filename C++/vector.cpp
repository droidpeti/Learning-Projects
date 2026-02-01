#include <iostream>
#include <algorithm>
#include <vector>
using namespace std;

auto int_from( istream& in ) -> int { int x; in >> x; return x; }

int main(){
    cout << "Enter numbers, to stop enter an EOF (CTRL+D on Unix, CTRL+Z on Windows)\n";

    vector<int> v;

    while(cin){
        cout << "Please enter a number or EOF: ";
        int const x = int_from( cin );
        if( !cin.fail() ){
            v.push_back(x);
        }
    }

    sort(v.begin(), v.end());

    int const n = v.size();
    for(int i = 0; i < n; i++){
        cout << v[i] << "\n";
    }

    return 0;
}
