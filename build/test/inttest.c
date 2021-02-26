#include "../include/type.h"
#include <stdio.h>


void main (void) {

  int size_schar = sizeof(signed char);
  typedef signed char int8;

  if (size_schar == 1) {
    printf("Hey there!");
  }

  int8 bytenum = 68;

  printf("int8: %i\n", bytenum);

}
