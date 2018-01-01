
#ifndef TEST_UTILS
#define TEST_UTILS

#ifdef __cplusplus
extern "C" {
#endif

void todo(void);
void pass(void);
int fail_with_information(int code, char *information);
#define fail(code) fail_with_information((code), ("__FILE__ __FUNCTION__: __LINE__"))
#define FAIL fail(-1)

#ifdef __cplusplus
}
#endif /* __cplusplus */

#endif /* TEST_UTILS */
