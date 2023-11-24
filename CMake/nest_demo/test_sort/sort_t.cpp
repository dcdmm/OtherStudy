#include <iostream>
#include <vector>
#include "sort.h"

int main() {
    std::vector<int> arr = {12, 11, 13, 5, 6};

    std::cout << "Original array: ";
    for (int i : arr) {
        std::cout << i << " ";
    }
    std::cout << std::endl;

    insertionSort(arr);

    std::cout << "Sorted array: ";
    for (int i : arr) {
        std::cout << i << " ";
    }
    std::cout << std::endl;
}