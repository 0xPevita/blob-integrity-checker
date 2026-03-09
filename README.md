# blob-integrity-checker

> Fast SHA-256 integrity checker for files stored on Shelby Protocol.

![Rust](https://img.shields.io/badge/Rust-000000?style=flat-square&logo=rust&logoColor=white)
![Shelby](https://img.shields.io/badge/Shelby-Protocol-c6ff00?style=flat-square&labelColor=0a0a0f)

## Overview

Verifies local files match their on-chain Shelby hash ensuring nothing was tampered with after upload.

## Build
```bash
cargo build --release
```

## Usage
```bash
./blob-checker --path ./myfile.pdf
./blob-checker --path ./myfile.pdf --expected abc123...
./blob-checker --path ./datasets --recursive
```

## Output
✅ MATCH    ./myfile.pdf → a1b2c3d4...
❌ MISMATCH ./other.pdf  → got 99887766 expected aabbccdd

## Why Rust?

File hashing is I/O and CPU intensive. Rust delivers zero-cost abstractions with no GC pauses - critical for large ML datasets on Shelby.

## License
MIT
