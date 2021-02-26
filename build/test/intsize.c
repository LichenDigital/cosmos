#include <stdio.h>

void main () {
  int sizeshort = sizeof(short);
  int sizeint = sizeof(int);
  int sizelong = sizeof(long);
  int sizelonglong = sizeof(long long);

  printf("Size of short: %i\nSize of int: %i\nSize of long: %i\nSize of long long: %i\n",
   sizeshort,
   sizeint,
   sizelong,
   sizelonglong);
}