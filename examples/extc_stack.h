//#include <stdlib.h>
//#include <string.h>
#include "extc_rint.h"

#define stack_template_def(name, type)\
    typedef struct stack_##name {\
        usize capacity;\
        usize size;\
        type* data;\
    } stack_##name;\
\
    stack_##name stack_##name##_init(usize size, u8* result);\
    u8 stack_##name##_realloc(stack_##name * v, usize n);\
    u8 stack_##name##_push(stack_##name * v, type o);\
    u8 stack_##name##_push_ptr(stack_##name * v, type* o, usize c);\
    u8 stack_##name##_pop(stack_##name * v, type* e);\
    u8 stack_##name##_pop_ptr(stack_##name * v, usize c, type** e);\
    void stack_##name##_free(stack_##name * v);\
    void stack_##name##_clean(stack_##name * v);



#define stack_template_impl(name, type)\
    stack_##name stack_##name##_init(usize _size, u8* result) {\
        stack_##name res;\
        res.size = 0;\
        res.capacity = _size;\
        res.data = (type*)malloc(sizeof(type)*_size);\
        if ((void*)result != NULL) {\
            if ((void*)res.data == NULL) *result = 1;\
            else *result = 0;\
        }\
        return res;\
    }\
    \
    u8 stack_##name##_realloc(stack_##name * v, usize n){\
        if (v->capacity == n) return false;\
        type* new_data = (type*)realloc(v->data, n * sizeof(type));\
        if ((void*)new_data == NULL) return false;\
        v->data = new_data;\
        v->capacity = n;\
        return true;\
    }\
    \
    u8 stack_##name##_push(stack_##name * v, type o) {\
        if(v->capacity < v->size+1) return false;\
        v->data[v->size++] = o;\
        return true;\
    }\
    \
    u8 stack_##name##_push_ptr(stack_##name * v, type* o, usize c) {\
        if(v->capacity < v->size+c) return false;\
        memcpy(&v->data[v->size], o, sizeof(type)*c);\
        v->size += c;\
        return true;\
    }\
    \
    u8 stack_##name##_pop(stack_##name * v, type* e) {\
        if(v->size == 0) return false;\
        if ((void*)e != NULL) *e = v->data[--v->size];\
        return true;\
    }\
    u8 stack_##name##_pop_ptr(stack_##name * v, usize c, type** e) {\
        if(v->size < c) return false;\
        v->size -= c;\
        if ((void*)e != NULL) *e = &v->data[v->size];\
        return true;\
    }\
    \
    void stack_##name##_free(stack_##name * v) {\
        v->size = 0;\
        v->capacity = 0;\
        free(v->data);\
    }\
    void stack_##name##_clean(stack_##name * v) {\
        stack_##name##_free(v);\
        v->data = (type*)malloc(0);\
    }