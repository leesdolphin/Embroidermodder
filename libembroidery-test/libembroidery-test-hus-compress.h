
#ifndef LIBEMBROIDERY_TEST_HUS_COMPRESS
#define LIBEMBROIDERY_TEST_HUS_COMPRESS

#ifdef __cplusplus
extern "C" {
#endif

#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include "emb-compress.h"

/* Copied from format-hus.c */
unsigned char *husDecompressData_(unsigned char *input, int compressedInputLength, int decompressedContentLength);

/* Copied from format-hus.c */
unsigned char *husCompressData_(unsigned char *input, int decompressedInputSize, int *compressedSize);

void doTestCompression(unsigned char *input, size_t length);

#ifdef __cplusplus
}
#endif /* __cplusplus */

#endif /* LIBEMBROIDERY_TEST_HUS_COMPRESS */

/* kate: bom off; indent-mode cstyle; indent-width 4; replace-trailing-space-save on; */
