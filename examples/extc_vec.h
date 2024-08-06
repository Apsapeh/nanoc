#ifndef VECTOR_NOSTD
    #include <stdlib.h>
    #include <string.h>

    #define VECTOR_MALLOC(size) malloc(size)
    #define VECTOR_REALLOC(ptr, size) realloc(ptr, size)
    #define VECTOR_FREE(ptr) free(ptr)
    #define VECTOR_MEMMOVE(dest, src, size) memmove(dest, src, size)
    #define VECTOR_MEMCPY(dest, src, size) memcpy(dest, src, size)
#endif

#define true 1
#define false 0

#define vector_template_def(name, type)\
    typedef struct vec_##name {\
        unsigned long capacity;\
        unsigned long size;\
        type* data;\
    } vec_##name;\
\
    vec_##name vec_##name##_init();\
    unsigned char vec_##name##_reserve(vec_##name * v, unsigned long n);\
    unsigned char vec_##name##_insert(vec_##name * v, unsigned long n, type o);\
    unsigned char vec_##name##_insert_data(vec_##name * v, unsigned long n, type* d, unsigned long count);\
    unsigned char vec_##name##_push_back(vec_##name * v, type o);\
    unsigned char vec_##name##_append(vec_##name * v, vec_##name * v2);\
    unsigned char vec_##name##_append_data(vec_##name * v, type* d, unsigned long n);\
    type vec_##name##_pop_back(vec_##name * v);\
    unsigned char vec_##name##_erase(vec_##name * v, unsigned long index);\
    unsigned char vec_##name##_erase_range(vec_##name * v, unsigned long index, unsigned long cound);\
    unsigned char vec_##name##_shrink_to_fit(vec_##name * v);\
    void vec_##name##_free(vec_##name * v);\
    void vec_##name##_clean(vec_##name * v);



#define vector_template_impl(name, type)\
    vec_##name vec_##name##_init() {\
        vec_##name res;\
        res.size = 0;\
        res.capacity = 0;\
        res.data = (type*)VECTOR_MALLOC(0);\
        return res;\
    }\
    \
    unsigned char vec_##name##_reserve(vec_##name * v, unsigned long n){\
        if (v->capacity == n) return false;\
        type* new_data = (type*)VECTOR_REALLOC(v->data, n * sizeof(type));\
        if ((void*)new_data == NULL) return false;\
        v->data = new_data;\
        v->capacity = n;\
        return true;\
    }\
    \
    unsigned char vec_##name##_insert(vec_##name * v, unsigned long index, type o) {\
        if(v->capacity < v->size+1) \
            if (!vec_##name##_reserve(v, v->size+1)) return false;\
        VECTOR_MEMMOVE(v->data+index+1, v->data+index, (v->size-index)*sizeof(type));\
        v->data[index] = o;\
        ++v->size;\
        return true;\
    }\
    \
    unsigned char vec_##name##_insert_data(vec_##name * v, unsigned long index, type* d, unsigned long count) {\
        if(v->capacity < v->size+count) \
            if (!vec_##name##_reserve(v, v->size+count)) return false;\
        VECTOR_MEMMOVE(v->data+index+count, v->data+index, (v->size-index)*sizeof(type));\
        VECTOR_MEMCPY(v->data+index, d, count*sizeof(type));\
        v->size += count;\
        return true;\
    }\
    \
    unsigned char vec_##name##_push_back(vec_##name * v, type o) {\
        if(v->capacity < v->size+1) \
            if (!vec_##name##_reserve(v, v->size+1)) return false;\
        v->data[v->size++] = o;\
        return true;\
    }\
    \
    unsigned char vec_##name##_append(vec_##name * v, vec_##name * v2) {\
        if (!vec_##name##_reserve(v, v->size+v2->size)) return false;\
        VECTOR_MEMCPY(v->data + v->size, v2->data, v2->size * sizeof(type));\
        v->size = v->size+v2->size;\
        return true;\
    }\
    unsigned char vec_##name##_append_data(vec_##name * v, type* d, unsigned long n) {\
        if (!vec_##name##_reserve(v, v->size+n)) return false;\
        VECTOR_MEMCPY(v->data + v->size, d, n * sizeof(type));\
        v->size = v->size+n;\
        return true;\
    }\
    \
    type vec_##name##_pop_back(vec_##name * v) {\
        return v->data[--v->size];\
    }\
    \
    unsigned char vec_##name##_erase(vec_##name * v, unsigned long index) {\
        if (index >= v->size) return false;\
        VECTOR_MEMCPY(v->data+index, v->data+index+1, (v->size-index-1)*sizeof(type));\
        --v->size;\
        return true;\
    }\
    \
    unsigned char vec_##name##_erase_range(vec_##name * v, unsigned long index, unsigned long count) {\
        if (index+count > v->size) return false;\
        VECTOR_MEMCPY(v->data+index, v->data+index+count, (v->size-index-count)*sizeof(type));\
        v->size -= count;\
        return true;\
    }\
    \
    unsigned char vec_##name##_shrink_to_fit(vec_##name * v) {\
        return vec_##name##_reserve(v, v->size);\
    }\
    \
    void vec_##name##_free(vec_##name * v) {\
        v->size = 0;\
        v->capacity = 0;\
        VECTOR_FREE(v->data);\
        v = NULL;\
    }\
    void vec_##name##_clean(vec_##name * v) {\
        vec_##name##_free(v);\
        v->data = (type*)VECTOR_MALLOC(0);\
    }\
    
