# loupe
Low-latency and low-memory Rust package for scrubbing through large text files.

### run

Creates 1GB file and dumps into `data/`:
```
cargo run --release -- <name_of_sample_text_file_to_dump> <size_of_test_file_in_MB>
```