# DiveSync

Main project root directory for the Dive Sync project. This project aims to be
the first to create all in one place to create dive algorithms.

These algorithms are developed in the `core` package and built as a library. We aim to build ZHL16 many more decompression algorithms as well as tools such as best gas mix and many more scuba diving tools which can be important to any other package.

It also includes starter code for ffi functionality which can be used with WASM in the web and mobile native code.

## Components

### Core

The "core" package holds the fundamental decompression logic for scuba diving.

- **Decompression Algorithm**: Implements the core decompression algorithm for safe and accurate diving.

### FFI (Foreign Function Interface)

The "ffi" package provides a low-level interface for different platforms using Rust's FFI.

- **Platform Integration**: Integrates with various platforms for seamless interoperability.
- **Native Libraries**: Allows usage in languages like C, C++, and others through the FFI.

### API

The "api" package offers a higher-level API for convenient use on different platforms.

- **RESTful API**: Provides a RESTful API for interaction with external applications.
- **Platform Agnostic**: Designed to be easily integrated into various applications.

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

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

## Roadmap

- Implement additional features.
- Improve protocol efficiency.
- Enhance security measures.

License

This project is licensed under the MIT License.
