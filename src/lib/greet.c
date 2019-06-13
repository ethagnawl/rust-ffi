#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char *punctuate_greeting(char *name) {
    char c = '!';
    size_t len = strlen(name);
    char *punctuated_greeting = malloc(len + 1 + 1);

    strcpy(punctuated_greeting, name);

    punctuated_greeting[len] = c;
    punctuated_greeting[len + 1] = '\0';

    return punctuated_greeting;
}

void display_greeting(char *greeting) {
    printf("%s\n", greeting);
}
