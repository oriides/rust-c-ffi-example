#include <stdlib.h>

struct CStruct
{
    __int32_t i;
};

struct CStruct *new_c(__int32_t i)
{
    struct CStruct *s = malloc(sizeof(*s));
    if (s != NULL)
        s->i = i;
    return s;
}

void drop_c(struct CStruct *s)
{
    free(s);
}