#include "callrust.h"
#include <stdio.h>
#include <stdint.h>
#include <inttypes.h>

int main (void) {
    uint32_t numbers[6] = {1,2,3,4,5,6};
    uint32_t sum = sum_of_even(numbers, 6);
    printf("%d\n", sum);

    uint32_t count = hm_chars("The taö of Rust");
    printf("%d\n", count);

    char *song = batman_song(5);
    printf("%s\n", song);
    free_song(song);

    tuple_t initial = { .x = 10, .y = 20 };
    tuple_t new = flip_things_around(initial);
    printf("(%d,%d)\n", new.x, new.y);

    database_t *database = database_new();
    database_insert(database);
    uint32_t pop1 = database_query(database, "10186");
    uint32_t pop2 = database_query(database, "10852");
    database_free(database);
    printf("%d\n", pop2 - pop1);   
     
    print_hello_from_rust();

    match();
}