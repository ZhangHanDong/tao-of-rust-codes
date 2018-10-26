#include <inttypes.h>
#include <stdio.h>

uint32_t sum_of_even(const uint32_t *numbers, size_t length);

char * batman_song(uint8_t length);
void  free_song(char *);

uint32_t hm_chars(const char *str);

void print_hello_from_rust();

typedef struct {
    uint32_t x;
    uint32_t y;
} tuple_t;
tuple_t flip_things_around(tuple_t);

typedef struct database_S database_t;
database_t * database_new(void);
void database_free(database_t *);
void database_insert(database_t *);
uint32_t database_query(const database_t *, const char *zip);

void match();