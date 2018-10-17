#include "sorting.h"
void interop_sort(int numbers[], size_t size)
{
    int* start = &numbers[0];
    int* end = &numbers[0] + size;
    std::sort(start, end, [](int x, int y) {  return x > y; });
}