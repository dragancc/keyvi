# -*- coding: utf-8 -*-
# some common tools for tests

import contextlib
import os
import tempfile
import pykeyvi

@contextlib.contextmanager
def tmp_dictionary(compiler, file_name):

    tmp_dir = tempfile.gettempdir()
    fq_file_name = os.path.join(tmp_dir, file_name)
    compiler.Compile()
    compiler.WriteToFile(fq_file_name)
    del compiler
    d = pykeyvi.Dictionary(fq_file_name)
    yield d
    del d
    os.remove(fq_file_name)