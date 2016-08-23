from libcpp.string cimport string as libcpp_string
from libc.string cimport const_char


cdef extern from "dictionary/dictionary_types.h" namespace "keyvi::dictionary":
    cdef cppclass KeyOnlyDictionaryGenerator:
        KeyOnlyDictionaryGenerator() except +
        void Add(libcpp_string) except +
        void CloseFeeding()
        void WriteToFile(libcpp_string)