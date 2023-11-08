# Dive Sync

Main project root directory for the Dive Sync project. This project aims to be
the first to create all in one place to create dive algorithms.

These algorithms are developed in the `core` package and built as a library. We aim to build ZHL16 many more decompression algorithms as well as tools such as best gas mix and many more scuba diving tools which can be important to any other package.

It also includes starter code for ffi functionality which can be used with WASM in the web and mobile native code.

## Development

### Run tests

```
make test_all
```

1. Run all tests
2. Run crate tests

### Build binaries

1. Make all binaries, and move them to `./bin` directory

```
make install
```
