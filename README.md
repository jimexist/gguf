# gguf

[![Rust](https://github.com/Jimexist/gguf/actions/workflows/rust.yml/badge.svg)](https://github.com/Jimexist/gguf/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/d/gguf)](https://crates.io/crates/gguf)
[![docs.rs](https://img.shields.io/docsrs/gguf)](https://docs.rs/gguf/)

A small utility library for parsing [GGUF](https://github.com/ggerganov/ggml/blob/master/docs/gguf.md) file info. See also [GGML](https://github.com/ggerganov/ggml) library.

## Running locally

```bash
$ cargo run --features bin -q -- --help
A small utility to parse GGUF files

Usage: gguf-info [OPTIONS] <PATH>

Arguments:
  <PATH>  The path to the file to read

Options:
      --read-buffer-size <READ_BUFFER_SIZE>  Size of read buffer (grows linearly) [default: 1000000]
  -t, --output-format <OUTPUT_FORMAT>        [default: table] [possible values: yaml, json, table]
  -h, --help                                 Print help
  -V, --version                              Print version
```

```bash
# check with your own gguf file
$ cargo run --features bin -- ~/GitHub/llama/llama-2-7b/ggml-model-Q4_0.gguf
Metadata:
+----+----------------------------------------+---------------+-----------------------+
| #  | Key                                    | Type          | Value                 |
+=====================================================================================+
| 1  | general.architecture                   | String        | llama                 |
|----+----------------------------------------+---------------+-----------------------|
| 2  | general.name                           | String        | LLaMA v2              |
|----+----------------------------------------+---------------+-----------------------|
| 3  | llama.context_length                   | Uint32        | 4096                  |
|----+----------------------------------------+---------------+-----------------------|
| 4  | llama.embedding_length                 | Uint32        | 4096                  |
|----+----------------------------------------+---------------+-----------------------|
| 5  | llama.block_count                      | Uint32        | 32                    |
|----+----------------------------------------+---------------+-----------------------|
| 6  | llama.feed_forward_length              | Uint32        | 11008                 |
|----+----------------------------------------+---------------+-----------------------|
| 7  | llama.rope.dimension_count             | Uint32        | 128                   |
|----+----------------------------------------+---------------+-----------------------|
| 8  | llama.attention.head_count             | Uint32        | 32                    |
|----+----------------------------------------+---------------+-----------------------|
| 9  | llama.attention.head_count_kv          | Uint32        | 32                    |
|----+----------------------------------------+---------------+-----------------------|
| 10 | llama.attention.layer_norm_rms_epsilon | Float32       | 0.00001               |
|----+----------------------------------------+---------------+-----------------------|
| 11 | general.file_type                      | Uint32        | 2                     |
|----+----------------------------------------+---------------+-----------------------|
| 12 | tokenizer.ggml.model                   | String        | llama                 |
|----+----------------------------------------+---------------+-----------------------|
| 13 | tokenizer.ggml.tokens                  | Array (32000) | <unk>, <s>, </s>, ... |
|----+----------------------------------------+---------------+-----------------------|
| 14 | tokenizer.ggml.scores                  | Array (32000) | 0, 0, 0, ...          |
|----+----------------------------------------+---------------+-----------------------|
| 15 | tokenizer.ggml.token_type              | Array (32000) | 2, 3, 3, ...          |
|----+----------------------------------------+---------------+-----------------------|
| 16 | general.quantization_version           | Uint32        | 2                     |
+----+----------------------------------------+---------------+-----------------------+
Tensors:
+-----+---------------------------+------+---------------+------------+
| #   | Name                      | Type | Dimensions    | Offset     |
+=====================================================================+
| 1   | token_embd.weight         | Q4_0 | [4096, 32000] | 0          |
|-----+---------------------------+------+---------------+------------|
| 2   | output_norm.weight        | F32  | [4096]        | 73728000   |
|-----+---------------------------+------+---------------+------------|
| 3   | output.weight             | Q6K  | [4096, 32000] | 73744384   |
|-----+---------------------------+------+---------------+------------|
```
