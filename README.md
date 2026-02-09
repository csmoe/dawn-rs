# dawn-rs

Rust bindings and wrappers for Dawn WebGPU.

## Status

This project is under active development and **not approved for production use**.

## Build Requirements

- Dawn build artifacts are expected under `DAWN_ROOT`:
  - `include/webgpu/webgpu.h`
  - `lib/libwebgpu_dawn.a`
- Dawn JSON schema is expected under `DAWN_JSON`:
  - `dawn.json`
- On Windows, `RUSTFLAGS="-Clink-arg=onecore_apiset.lib -Clink-arg=dxguid.lib"` is needed to run the example.

## Example

```bash
DAWN_ROOT=/dawn_release_path DAWN_JSON=/path/to/dawn.json cargo run --example adapter_info
```

## Upstream References (Dawn)

- `dawn.json` (API schema): [dawn.json](https://dawn.googlesource.com/dawn/+/HEAD/src/dawn/dawn.json)
- Generator design notes: [docs/codegen.md](https://dawn.googlesource.com/dawn/+/HEAD/docs/dawn/codegen.md)
- C++ wrapper template: [generator/templates/api_cpp.h](https://dawn.googlesource.com/dawn/+/HEAD/generator/templates/api_cpp.h)

## License

This project is licensed under the BSD 3-Clause "New" or "Revised" License. See [LICENSE](LICENSE).
