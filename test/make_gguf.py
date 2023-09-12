# /usr/bin/env python3
from gguf import GGUFWriter
import numpy as np


def write_gguf_file(out_path: str):
    """
    write a minimalist GGUF file, example taken from
    https://github.com/ggerganov/llama.cpp/blob/master/gguf-py/gguf/gguf.py
    """
    gguf_writer = GGUFWriter(out_path, "llama")

    gguf_writer.add_architecture()
    gguf_writer.add_block_count(12)
    gguf_writer.add_uint32("answer", 42)
    gguf_writer.add_float32("answer_in_float", 42.0)
    gguf_writer.add_custom_alignment(64)

    tensor1 = np.ones((32,), dtype=np.float32) * 100.0
    tensor2 = np.ones((64,), dtype=np.float32) * 101.0
    tensor3 = np.ones((96,), dtype=np.float32) * 102.0

    gguf_writer.add_tensor("tensor1", tensor1)
    gguf_writer.add_tensor("tensor2", tensor2)
    gguf_writer.add_tensor("tensor3", tensor3)

    gguf_writer.write_header_to_file()
    gguf_writer.write_kv_data_to_file()
    gguf_writer.write_tensors_to_file()

    gguf_writer.close()


if __name__ == "__main__":
    import sys

    assert len(sys.argv) == 2, "Usage: make_gguf.py <file>"

    write_gguf_file(sys.argv[1])
