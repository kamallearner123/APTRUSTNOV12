#include <execinfo.h>
#include <stdio.h>
#include <stdlib.h>

void print_call_stack() {
    void *buffer[100];
    int size;

    // Get the call stack
    size = backtrace(buffer, 100);

    // Convert addresses to symbols
    char **symbols = backtrace_symbols(buffer, size);
    if (symbols == NULL) {
        perror("backtrace_symbols");
        exit(EXIT_FAILURE);
    }

    printf("Call stack:\n");
    for (int i = 0; i < size; i++) {
        printf("%s\n", symbols[i]);
    }

    free(symbols);
}

void functionC() {
    print_call_stack();
}

void functionB() {
    functionC();
}

void functionA() {
    functionB();
}

int main() {
    functionA();
    return 0;
}
