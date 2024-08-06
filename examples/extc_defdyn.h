/* Define attrib for cross lib */

#ifndef __EXTC_DEFDYN
#define __EXTC_DEFDYN

#if (defined(__GNUC__) /*&& defined(__clang__)*/)
    #define _packed __attribute__((__packed__))
    #define _export __attribute__((visibility("default")))
#elif (defined(_MSC_VER))
    #define _packed 
    #ifdef __NCVM_DYN_LIB_EXPORT
        #define _export __declspec(dllexport)
    #else
        #define _export __declspec(dllimport)
    #endif
#else
    #define _packed
    #define _export
#endif

#endif