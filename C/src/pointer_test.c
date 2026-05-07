#include <stdio.h>

int main(){
    int var = 100;
    int* ptr = &var;

    printf("%d", *ptr); // Result: 100
    printf("\n");
    printf("%p", *ptr); // Result: 0x64 

    return 0;
}
