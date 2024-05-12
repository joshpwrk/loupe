# loupe
Low-latency and low-memory Rust package for scrubbing through large text files.

### run

1. creates 1GB file
2. dumps into `data/`:
3. then reads 1000th line

```
cargo run --release -- <name_of_sample_text_file_to_dump> <size_of_test_file_in_MB>
```