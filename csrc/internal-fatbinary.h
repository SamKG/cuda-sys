#include <fatbinary_section.h>

typedef __attribute__((aligned(8))) struct fatBinaryHeader
{
    unsigned int           magic;
    unsigned short         version;
    unsigned short         headerSize;
    unsigned long long int fatSize;
} fatBinaryHeader;

_Static_assert(_Alignof(fatBinaryHeader) == 8, "fatBinaryHeader must be 8-byte aligned");
