# dawn-rs

Rust bindings and thin wrappers for Dawn WebGPU.

## Status

Active development. **Not approved for production use.**

## Requirements

- Dawn build or prebuilt release containing:
  - `include/webgpu/webgpu.h`
  - `lib/libwebgpu_dawn.a` (or platform equivalent)
- `DAWN_ROOT` points to the Dawn artifacts directory.

## Generate Bindings

Inputs
- `DAWN_JSON` points to `dawn.json` from Dawn source.
- `--api-header` points to `include/webgpu/webgpu.h` from the build/prebuilt artifacts.

Commands
- Windows
  - `cargo run -p dawn-codegen --bin dawn_codegen -- --dawn-json <path_to_dawn_json> --api-header <path_to_webgpu.h> --out-dir src/generated`
- macOS
  - `cargo run -p dawn-codegen --bin dawn_codegen -- --dawn-json <path_to_dawn_json> --api-header <path_to_webgpu.h> --out-dir src/generated --clang-arg --sysroot=$(xcrun --show-sdk-path --sdk macosx)`

Notes
- Generated `ffi.rs` is split per OS/arch under `src/generated/ffi/`, with `src/generated/ffi.rs` selecting the active target.
- `build.rs` only emits link directives; codegen is run explicitly.

## Examples

```bash
DAWN_ROOT=/path/to/dawn cargo run --example adapter_info
```

## Upstream References

- Dawn API schema: [dawn.json](https://dawn.googlesource.com/dawn/+/HEAD/src/dawn/dawn.json)
- Generator design notes: [docs/codegen.md](https://dawn.googlesource.com/dawn/+/HEAD/docs/dawn/codegen.md)
- C++ wrapper template: [generator/templates/api_cpp.h](https://dawn.googlesource.com/dawn/+/HEAD/generator/templates/api_cpp.h)

## License

BSD 3-Clause "New" or "Revised" License. See [LICENSE](LICENSE).
