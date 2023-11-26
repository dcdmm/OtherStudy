#include <vector>
#include <iostream>
#include "sort.h"
#include "calc.h"

void insertionSort(std::vector<int> &arr) {
    int a = 100;
    int b = 100;
    std::cout << "sum a + b = " << add(a, b)<< std::endl;


    auto n = arr.size();
    for (int i = 1; i < n; i++) {
        int key = arr[i];
        int j = i - 1;

        while (j >= 0 && arr[j] > key) {
            arr[j + 1] = arr[j];
            j = j - 1;
        }
        arr[j + 1] = key;
    }
}