#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include "emb-compress.h"
#include "libembroidery-test-hus-compress.h"
#include "test-utils.h"

/* Copied from format-hus.c */
unsigned char *husDecompressData_(unsigned char *input, int compressedInputLength, int decompressedContentLength)
{
    unsigned char *decompressedData = (unsigned char *)malloc(sizeof(unsigned char) * decompressedContentLength);
    if (!decompressedData)
    {
        fail_with_information(1, "format-hus.c husDecompressData(), cannot allocate memory for decompressedData\n");
        return NULL;
    }
    husExpand((unsigned char *)input, decompressedData, compressedInputLength, 10);
    return decompressedData;
}

/* Copied from format-hus.c */
unsigned char *husCompressData_(unsigned char *input, int decompressedInputSize, int *compressedSize)
{
    unsigned char *compressedData = (unsigned char *)malloc(sizeof(unsigned char) * decompressedInputSize * 2);
    if (!compressedData)
    {
        fail_with_information(1, "format-hus.c husCompressData(), cannot allocate memory for compressedData\n");
        return NULL;
    }
    *compressedSize = husCompress(input, (unsigned long)decompressedInputSize, compressedData, 10, 0);
    return compressedData;
}

void doTestCompression(unsigned char *input, size_t length)
{
    unsigned char *compressed;
    unsigned char *decompressed;
    int compressedLength, decompressedLength = length;
    size_t i;
    char failureTmp[128];
    printf("Compressing data\n");
    compressed = husCompressData_(input, length, &compressedLength);
    if (compressed == 0)
    {
        fail_with_information(2, "Compression failed.");
        return;
    }
    printf("Compressed data: ");
    for (i = 0; i < compressedLength; i++) {
        printf("%02x ", compressed[i]);
    }
    printf("\nDecompressing data\n");
    decompressed = husDecompressData_(compressed, compressedLength, decompressedLength);
    free(compressed);
    if (decompressed == 0)
    {
        fail_with_information(2, "Decompression failed.");
        return;
    }
    printf("Decompressed data: ");
    for (i = 0; i < decompressedLength; i++)
    {
        printf("%02x ", decompressed[i]);
    }
    printf("\n");
    for (i = 0; i < length; i++)
    {
        if (input[i] != decompressed[i])
        {
            printf("Input & Decompressed mismatch at %li. %02x != %02x\n", i, input[i], decompressed[i]);
            fail(3);
        }
    }
    free(decompressed);
}

/* kate: bom off; indent-mode cstyle; indent-width 4; replace-trailing-space-save on; */
