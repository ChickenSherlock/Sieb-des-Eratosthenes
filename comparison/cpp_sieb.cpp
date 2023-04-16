//
// Created by Benedikt Karli on 15.04.23.
//

#include<iostream>
#include<vector>
#include<cmath>
#include<chrono>

using namespace std;

std::vector<int> basic_sieve(int n) {
    std::vector<bool> bool_array;
    for (int i = 0; i < n; i++) {
        bool_array.push_back(true);
    }
    bool_array[0] = false;
    bool_array[1] = false;

    for (int number=2; number<n; number++){
        if (bool_array[number]){
            for (int i = number; i<n;i++){
                if((i % number) == 0 && (number != i)){
                    bool_array[i] = false;
                }
            }

        }
    }

    std::vector<int> output_array = {};
    for (int i = 0; i < bool_array.size(); i++){
        if (bool_array[i]){
           output_array.push_back(i);
        }
    }
    return output_array;
}

std::vector<int> improved_sieve(int n) {
    std::vector<bool> bool_array;
    for (int i = 0; i < n; i++) {
        bool_array.push_back(true);
    }
    bool_array[0] = false;
    bool_array[1] = false;

    for (int number = 2; number<floor(sqrt(n)); number++){
        if (bool_array[number]){
            for (int i = number*number; i<n; i=i+number){
                bool_array[i] = false;
            }
        }
    }

    std::vector<int> output_array = {};
    for (int i = 0; i < bool_array.size(); i++){
        if (bool_array[i]){
            output_array.push_back(i);
        }
    }
    return output_array;
}

int main() {

    int n = 100000000;

    auto start = std::chrono::high_resolution_clock::now();
    std::vector<int> out = basic_sieve(n);

    auto end = std::chrono::high_resolution_clock::now();
    auto duration = std::chrono::duration_cast<std::chrono::microseconds>(end - start);

    cout << duration.count() << endl;
    auto start2 = std::chrono::high_resolution_clock::now();
    std::vector<int> out2 = improved_sieve(n);
    auto end2 = std::chrono::high_resolution_clock::now();
    auto duration2 = std::chrono::duration_cast<std::chrono::microseconds>(end2 - start2);
    cout << duration2.count() << endl;
    return 0;
}