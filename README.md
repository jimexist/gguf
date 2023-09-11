# gguf

[![Rust](https://github.com/Jimexist/gguf/actions/workflows/rust.yml/badge.svg)](https://github.com/Jimexist/gguf/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/d/gguf)](https://crates.io/crates/gguf)
[![docs.rs](https://img.shields.io/docsrs/gguf)](https://docs.rs/gguf/)


A small utility library for parsing [GGUF](https://github.com/philpax/ggml/blob/gguf-spec/docs/gguf.md) file info. See also [GGML](https://github.com/ggerganov/ggml) library.

## Running locally

```bash
# check with your own gguf file
cargo run --features bin -- ~/GitHub/llama/llama-2-7b/ggml-model-Q4_0.gguf
```
