#include <iostream>
#include "calc.h"

int main() {
    int a = 20;
    int b = 12;

    std::cout << add(a, b) << std::endl;
    std::cout << subtract(a, b) << std::endl;
    std::cout << multiply(a, b) << std::endl;
    std::cout << divide(a, b) << std::endl;
}