#include <iostream>

int main() {
#ifdef DEBUG
    std::cout << "******DEBUG*****\n";
#endif
    for (int i = 0; i <= 5; ++i) {
        std::cout << "hello cmake!" << std::endl;
    }
}

