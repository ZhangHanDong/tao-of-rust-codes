#!/usr/bin/env python3
import sys, ctypes
from ctypes import c_char_p, c_uint32, Structure, POINTER

prefix = {'win32': ''}.get(sys.platform, '../target/debug/lib')
extension = {'darwin': '.dylib', 'win32': '.dll'} \
    .get(sys.platform, '.so')

class DatabaseS(Structure):
    pass

lib = ctypes.cdll.LoadLibrary(prefix + "callrust" + extension)

# 必须以DatabaseS作为参数，否则会产生空指针
lib.database_new.restype = POINTER(DatabaseS)
lib.database_free.argtypes = (POINTER(DatabaseS), )
lib.database_insert.argtypes = (POINTER(DatabaseS), )
lib.database_query.argtypes = (POINTER(DatabaseS), c_char_p)
lib.database_query.restype = c_uint32

class Database:
    def __init__(self):
        self.obj = lib.database_new()
    def __enter__(self):
        return self
    def __exit__(self, exc_type, exc_value, traceback):
        lib.database_free(self.obj)
    def insert(self):
        lib.database_insert(self.obj)
    def query(self, zip):
        return lib.database_query(self.obj, zip.encode('utf-8'))
with Database() as database:
    database.insert()
    pop1 = database.query("10186")
    pop2 = database.query("10852")
    print(pop2 - pop1)