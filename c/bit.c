// This code will intro basic bit manipulation

// imports
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

// declared functions
int and(int a, int b);
void and_example();

// main function
int main() {
    and_example();
    return EXIT_SUCCESS;
}

// and operation
int and(int a, int b) {
    // return result for __ & __
    return a & b;
}

// or operation
int or(int a, int b) {
    // return result for __ | __
    return a | b;
}

// xor operation
int xor(int a, int b) {
    // return result for __ ^ __
    return a ^ b;
}

// and example
void and_example() {
    // define a
    int a = 4; // 0100
    
    // define b
    int b = 3; // 0011

    // get and result between the two
    int result = and(a, b); // 0000

    // print result
    fprintf(stdout, "%d & %d == %d\n", a, b, result);
}


