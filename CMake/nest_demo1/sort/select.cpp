#include <vector>
#include <iostream>
#include "sort.h"
#include "calc.h"

void selectionSort(std::vector<int> &arr) {
    int a = 100;
    int b = 100;
    std::cout << "sum a + b = " << add(a, b)<< std::endl;

    std::size_t n = arr.size();
    for (int i = 0; i < n - 1; i++) {
        int minIndex = i;
        for (int j = i + 1; j < n; j++) {
            if (arr[j] < arr[minIndex]) {
                minIndex = j;
            }
        }
        std::swap(arr[i], arr[minIndex]);
    }
}

