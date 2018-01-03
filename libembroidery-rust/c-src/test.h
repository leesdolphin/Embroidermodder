/*! @file format-xxx.h */
#ifndef TEST_H
#define TEST_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stdio.h>

unsigned int failure_count;

#define assert_eq(a, b, error)                                          \
    {                                                                   \
        if (a != b)                                                     \
        {                                                               \
            printf("Error on %s: %i >| %s", __FILE__, __LINE__, error); \
            failure_count++;                                            \
        }                                                               \
    }

#ifdef __cplusplus
}
#endif /* __cplusplus */

#endif /* TEST_H */

/* kate: bom off; indent-mode cstyle; indent-width 4; replace-trailing-space-save on; */
