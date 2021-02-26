/* Tests */

/* Determine the size of standard base data types */
struct type {
  int size_char; /* Signed Char. Normally one byte. Must be explisitly signed or unsigned for it to avoid
  being encoded to an ASCII character in some functions. */
  int size_short; /* Signed Short int. Normally 2 bytes. */
  int size_int; /* Signed Int. Normally 2 or 4 bytes. */
  int size_long; /* Signed Long int. Normally 4 or 8 bytes. */
  int size_longlong; /* Signed Long long int. Normally 8 bytes. */

  int size_uchar; /* Unsigned char. Normally one byte. Must be explicityly signed or unsigned for it to
  avoid being encoded to an ASCII character in some functions. */
  int size_ushort; /* Unsigned short int. Normally 2 bytes. */
  int size_uint; /* Unsigned int. Normally 2 or 4 bytes */
  int size_ulong; /* Unsigned long int. Normally 4 or 8 bytes */
  int size_ulonglong; /* Unsigned long long int. Normally 8 bytes */

  int size_float; /* Float. Normally 4 bytes. */
  int size_double; /*Double. Floating point storage, normally 8 bytes, or double the float size. */
  int size_longdouble; /* Long double. Floating point storage, normally 10 bytes. */
};
