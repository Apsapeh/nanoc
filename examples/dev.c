#include "dev.h"

#define A 10
#define B 20 + A
#define Aboba 30 + B

/*#define\
 multiline(a\
, b) "oeu   31  eou\a" \
    a + b*/
 
#define M(a, b) a + b\
+3

#define M2(int, b) int - "M(10, Aboba)"

#define new_name(name) aboba__##name##__aboba



const char* str = "Hello  oeu";

//#define Err
#define Err2
#ifdef Err
#ifndef Err2
#error Всё       Плохо eсдд    сдфдсф#есфг
#endif
int b = 15;
#endif
void print(const char * a) {
    //Aboba;
    //GLOBAL;
    TEST_INCLUDE
    M(1, 2);
    int b = -15;
    0x125;
    float c = 15.01;
    //new_name(Tofik);
    /*__asm__ {
        read b char

    }*/
}

int func            (int /**/ a) {
    //return M2(5 + func(10), 1);
    //return M2(5, 1);
    return 0;
}

int main() {
    return func(Aboba);
}
