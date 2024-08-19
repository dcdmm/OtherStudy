#include <iostream>

int main() {
#ifdef DEBUG_PRINT
    std::cout << "******DEBUG PRINT IMFOMATION*****\n";
#endif
    for (int i = 0; i <= 5; ++i) {
        std::cout << "hello cmake!" << std::endl;
    }
}

