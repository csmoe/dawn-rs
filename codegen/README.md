# Dawn JSON Parser

A Rust parser for the Dawn WebGPU API definition format (`dawn.json`). This parser handles the complete schema as documented in [Dawn's codegen documentation](https://github.com/google/dawn/blob/main/docs/dawn/codegen.md).

## Overview

The Dawn project uses JSON files to define the WebGPU API with additional metadata for code generation. This parser provides a complete Rust implementation that can:

- Parse `dawn.json` files into strongly-typed Rust structures
- Filter definitions by tags (dawn, emscripten, native, etc.)
- Extract specific categories of definitions (enums, structures, objects, etc.)
- Handle all 12 definition categories including callback functions
- Support flexible length values (string references or numeric literals)
- Convert between different naming conventions
- Provide a foundation for Rust code generation

## Implementation Notes

This parser is designed to handle the complete Dawn JSON schema as of 2024, including:

- All 12 definition categories and their specific properties
- Tag-based filtering system
- Extensible structure chaining (boolean and directional)
- Dawn-specific extensions and metadata
- Emscripten-specific annotations
- Wire protocol annotations
- Callback function definitions (both `callback` and `callback function` categories)

## Parser Completeness

The parser has been validated against the real dawn.json file and successfully handles:

- **5800+ total definitions** across all categories
- **Complex extensible structures** with both boolean (`true`/`false`) and directional (`"in"`/`"out"`) extensible types
- **Platform-specific definitions** with sophisticated tag filtering
- **All WebGPU core objects** (Device, Adapter, Buffer, Texture, etc.)
- **Complete method signatures** with proper argument handling
- **Callback patterns** used throughout the WebGPU API

The parsed data structures are designed for code generation and provide a complete foundation for creating Rust bindings, C++ headers, or bindings for other target languages.
