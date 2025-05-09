/* Defines a variety of number types that are fixed
sizes in bits, signed and unsigned, accross multiple platforms */


/*

C Data Types
  These may vary in size depending on the platform. That is why compilers, with the knowledge of platform specific sizing needs
  normally define strictly sized 

signed char
char
  - Normally 1 byte. Must be explisitly signed or unsigned for it to avoid
  being encoded to an ASCII character in some functions.

signed short int
short int
short
  - Normally 2 bytes.

signed int
int
  - Normally 2 or 4 bytes.

signed long int
long int
long
  - Normally 4 or 8 bytes. 

signed long long int
signed long long
long long
  - Normally 8 bytes.

unsigned char
  - Normally 1 byte. Must be explicityly signed or unsigned for it to
    avoid being encoded to an ASCII character in some functions.\

unsigned short int
unsigned short
  - Normally 2 bytes.

unsigned int
  - Normally 2 or 4 bytes

unsigned long int
unsigned long
  - Normally 4 or 8 bytes

unsigned long long int
unsigned long
  - Normally 8 bytes

*/

#ifndef COSMOS_STDINT_H
  #define COSMOS_STDINT_H

  #ifdef _MSC_VER
    #include "./windows/stdint.h"
  #else
    #include <stdint.h>
  #endif

#endif /* COSMOS_STDINT_H */