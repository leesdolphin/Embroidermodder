
#ifndef TEST_UTILS
#define TEST_UTILS

#ifdef __cplusplus
extern "C" {
#endif

#include <stdio.h>

void todo(void);
void pass(void);
int do_fail(int code, char *file, int line, const char* function, char *information);
#define fail_with_information(code, information) do_fail((code), __FILE__, __LINE__, "", information)
#define fail(code) fail_with_information((code), "")
#define FAIL fail(-1)

#ifdef __cplusplus
}
#endif /* __cplusplus */

#endif /* TEST_UTILS */
