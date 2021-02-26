#include <stdio.h>
#include <stdint.h>

typedef char int8;
typedef signed char sint8;
typedef unsigned char uint8;

int sizeint8 = sizeof(int8);
int sizesint8 = sizeof(sint8);
int sizeuint8 = sizeof(uint8);

void main (void) {

	printf("Size of int8: %i\nSize of sint8: %i\nSize of uint8: %i\n", sizeint8, sizesint8, sizeuint8);

  /* Set newly defined types to values to see how they work */
	/*int8 numba1 = 8;*/
  sint8 numba2 = 13;
  uint8 numba3 = 159;

  printf("Set newly defined one byte integer types to values to see if they play nicely...\n");
  printf("numba2: %i\nnumba3; %i\n", numba2, numba3); /* This doesn't work in the current implimentation */

  int8_t stdnumba1 = 123;
  int sizeint8_t = sizeof(stdnumba1);

  printf("Size of int8_t: %i\nstdnumba1: %i\n", sizeint8_t, stdnumba1);

  typedef __INT8_TYPE__ gccint8;
  int size_gccint8 = sizeof(gccint8);
  gccint8 gccnumba1 = 113;

  printf("Size of gccint8: %i\ngccnumba1: %i\n", size_gccint8, gccnumba1);

}
