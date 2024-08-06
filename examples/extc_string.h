#include "extc_vec.h"

#ifndef STRING_NOSTD
    #include <string.h>
    #define STRING_STRLEN(str) strlen(str)
    #define STRING_STRNCMP(str1, str2, n) strncmp(str1, str2, n)
#endif 

#define true 1
#define false 0


vector_template_def(__char, char);


typedef struct string {
    unsigned long size;
    char* str;
    vec___char cstr_vec;
} string;


vector_template_def(string, string)


string string_new();
string string_from(const char* str);
string string_from_n(const char* str, unsigned long n);
string stings_concat(string* self, string* vstr);
string string_concat_str(string* self, const char* str);
unsigned char string_insert(string* self, unsigned long index, char c);
unsigned char string_insert_str(string* self, unsigned long index, const char* str);
unsigned char string_erase(string* self, unsigned long index);
unsigned char string_erase_range(string* self, unsigned long index, unsigned long count);

void string_strip(string* self);
void string_replace(string* self, const char* pat, const char* rep);
vec_string string_split(string* self, const char* pat, unsigned char skip_empty_lines);

void string_free(string* self);
void free_vec_string(vec_string* vec);



#define string_template_impl() \
    vector_template_impl(__char, char) \
    vector_template_impl(string, string) \
    string string_new() {\
        string r;\
        r.size = 0;\
        r.cstr_vec = vec___char_init();\
        vec___char_push_back(&r.cstr_vec, '\0');\
        r.str = r.cstr_vec.data;\
        return r;\
    }\
    \
    string string_from(const char* str) {\
        return string_from_n(str, STRING_STRLEN(str));\
    }\
    \
    string string_from_n(const char* str, unsigned long n) {\
        string r;\
        r.cstr_vec = vec___char_init();\
        vec___char_append_data(&r.cstr_vec, (char*) str, n);\
        if (r.cstr_vec.data[r.cstr_vec.size-1] != '\0')\
            vec___char_push_back(&r.cstr_vec, '\0');\
        r.size = r.cstr_vec.size-1;\
        r.str = r.cstr_vec.data;\
        return r;\
    }\
    \
    unsigned char string_insert(string* self, unsigned long index, char c) {\
        if (index > self->size) return false;\
        vec___char_insert(&self->cstr_vec, index, c);\
        self->size = self->cstr_vec.size-1;\
        return true;\
    }\
    \
    unsigned char string_insert_str(string* self, unsigned long index, const char* str) {\
        if (index > self->size) return false;\
        vec___char_insert_data(&self->cstr_vec, index, (char*) str, STRING_STRLEN(str));\
        self->size = self->cstr_vec.size-1;\
        return true;\
    }\
    \
    unsigned char string_erase(string* self, unsigned long index) {\
        if (index >= self->size) return false;\
        vec___char_erase(&self->cstr_vec, index);\
        self->size = self->cstr_vec.size-1;\
        return true;\
    }\
    unsigned char string_erase_range(string *self, unsigned long index, unsigned long count) {\
        if (index+count > self->size) return false;\
        vec___char_erase_range(&self->cstr_vec, index, count);\
        self->size = self->cstr_vec.size-1;\
        return true;\
    }\
    \
    vec_string string_split(string* str, const char* pat, unsigned char skip_empty_lines) {\
        vec_string lines = vec_string_init();\
        unsigned long pat_len = STRING_STRLEN(pat);\
        unsigned long i;\
        unsigned long last = 0;\
        for (i=0; i<str->cstr_vec.size; ++i) {\
            if (STRING_STRNCMP(str->str+i, pat, pat_len) == 0 || str->str[i] == '\0') {\
                if (skip_empty_lines && i == last) {\
                    last = i+pat_len;\
                    continue;\
                }\
                string line = string_from_n(str->str+last, i-last);\
                vec_string_push_back(&lines, line);\
                last = i+pat_len;\
                i += pat_len-1;\
            }\
        }\
        return lines;\
    }\
    \
    void strip_string(string* str) {\
        char* ch;\
        for (ch=str->str; ch-str->str < str->cstr_vec.size; ++ch)\
            if (*ch == ' ' || *ch == '\t' || *ch == '\n') {\
                string_erase(str, ch - str->str);\
                break;\
            }\
        \
        for (ch = str->str + str->size; ch-str->str > 0; --ch)\
            if (*ch == ' ' || *ch == '\t' || *ch == '\n') {\
                string_erase(str, ch - str->str);\
                break;\
            }\
        \
    }\
    \
    void string_replace(string* str, const char* pat, const char* rep) {\
        unsigned long pat_len = STRING_STRLEN(pat);\
        unsigned long rep_len = STRING_STRLEN(rep);\
        unsigned long i;\
        for (i=0; i<str->size; ++i) {\
            if (STRING_STRNCMP(str->str+i, pat, pat_len) == 0) {\
                string_erase_range(str, i, pat_len);\
                string_insert_str(str, i, rep);\
                i += rep_len-1;\
            }\
        }\
    }\
    \
    void string_free(string* self) {\
        vec___char_free(&self->cstr_vec);\
        self->size = 0;\
        self->str = NULL;\
        self = NULL;\
    }\
    \
    void free_vec_string(vec_string* vec) {\
        unsigned long i;\
        for (i=0; i<vec->size; ++i)\
            string_free(&vec->data[i]);\
        vec_string_free(vec);\
    }\
