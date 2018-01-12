/* An test file to ensure that the Rust bindings and headers have been loaded and configured correctly from QMake. */

#include "test.h"

#include "pattern.h"

int main(int argc, const char *argv[])
{
    EmbColor *macro = embColor_create(10, 11, 12);
    assert_eq(macro->r, 10, "Red is not equal.")
    assert_eq(macro->g, 11, "Red is not equal.")
    assert_eq(macro->b, 12, "Red is not equal.")
    free(macro);

    EmbPattern *pattern = embPattern_create();
    embPattern_scale(pattern, 2.0);
    embPattern_free(pattern);

    if (failure_count > 0)
    {
        return 1;
    } else {
        return 0;
    }
}
