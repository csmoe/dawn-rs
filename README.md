# dawn-rs

Rust bindings for Dawn WebGPU, plus `dawn-wgpu` compatibility wrappers for `wgpu`.

## Project Layout

- `dawn-rs` (`src/`): generated + thin handwritten bindings over Dawn C API.
- `src/generated/api.rs`: target-independent safe wrapper generated from `dawn.json`.
- `src/ffi/<os>_<arch>.rs`: target-specific raw ABI generated from the release header.
- `dawn-wgpu` (`dawn-wgpu/`): `wgpu` custom backend compatibility layer.
- `codegen/`: generator for Rust bindings from `dawn.json` + `webgpu.h`.

## Requirements

- Rust stable toolchain.
- `DAWN_ROOT` pointing to a Dawn build/prebuilt root that contains dawn binaries and headers.

Example:

```bash
DAWN_ROOT=/path/to/dawn cargo check
```

## Link Modes

- Default: static Dawn linkage.
- Optional dynamic linkage: enable `dawn-dynamic` feature where needed.

## Quick Start

`dawn-rs` adapter info:

```bash
DAWN_ROOT=/path/to/dawn cargo run --example adapter-info
```

`dawn-rs` triangle:

```bash
DAWN_ROOT=/path/to/dawn cargo run --example triangle
```

`dawn-wgpu` triangle:

```bash
DAWN_ROOT=/path/to/dawn cargo run -p dawn-wgpu --example triangle
DAWN_ROOT=/path/to/dawn cargo run -p dawn-wgpu --example triangle -- --swiftshader-vk
```

Wire examples:

```bash
DAWN_ROOT=/path/to/dawn cargo run --example triangle-wire --features wire
DAWN_ROOT=/path/to/dawn cargo run -p dawn-wgpu --example nya-cat-wire --features wire
```

The `wire` feature requires a Dawn source checkout built with the `dawn_proc`, `webgpu_dawn`, and
`dawn_wire` targets. Official release prebuilt archives contain `webgpu_dawn` only and are not
sufficient. `DAWN_ROOT` must point at the source checkout containing `out/Release` or `out/Debug`.
The client and server must use the same `DAWN_VERSION`; the IPC handshake rejects mismatched wire
protocols. `Client::pump` and `Server::flush` return transport failures instead of deferring them to
shutdown. Creating a wire client is `unsafe`: Dawn uses a process-global proc table, and every
proxy derived from the client must be dropped before the client itself. Run native server and wire
client code in separate processes.

## Code Generation

Inputs:

- `--dawn-json`: Dawn schema file (`src/dawn/dawn.json` from Dawn source).
- `--api-header`: `include/webgpu/webgpu.h` from Dawn build/prebuilt.
- `--out-dir`: usually `src/generated`.
- `--target-os` / `--target-arch` (optional): override target file naming.

Host-target generation:

```bash
cargo run -p dawn-codegen --bin dawn_codegen -- \
  --dawn-json <path_to_dawn_json> \
  --api-header <path_to_webgpu_h> \
  --out-dir src/generated
```

Explicit single-target generation:

```bash
cargo run -p dawn-codegen --bin dawn_codegen -- \
  --dawn-json <path_to_dawn_json> \
  --api-header <path_to_webgpu_h> \
  --out-dir src/generated \
  --target-os linux \
  --target-arch x86_64
```

Notes:

- One invocation updates the shared safe API and one target-specific FFI file.
- Dispatch modules live in `src/generated/mod.rs` and `src/ffi/mod.rs`.
- Codegen is explicit and **not** run automatically by `cargo build`.
- `--skip-ffi` updates only `src/generated/api.rs` and does not require `--api-header`.
- `--target-os` and `--target-arch` select a real clang target; they do not merely rename output.
- Input-structure constructors require schema fields that have no default. Async callback methods
  require an explicit `CallbackMode`.

## Update Dawn Snapshot

Regenerate against latest Dawn release:

```bash
python3 scripts/update_dawn_release.py
```

Optional:

- `--force`: regenerate even if `DAWN_VERSION` is already current.
- `DAWN_TAGS`: override generator tags (default `dawn,native`).
- `DAWN_CODEGEN_TARGET_OS` / `DAWN_CODEGEN_TARGET_ARCH`: pin single target output.

## Upstream References

- Dawn schema: [dawn.json](https://github.com/google/dawn/blob/main/src/dawn/dawn.json)
- Dawn codegen docs: [docs/dawn/codegen.md](https://github.com/google/dawn/blob/main/docs/dawn/codegen.md)
- Dawn C++ API template: [generator/templates/api_cpp.h](https://github.com/google/dawn/blob/main/generator/templates/api_cpp.h)
- Dawn wire API: [include/dawn/wire](https://github.com/google/dawn/tree/main/include/dawn/wire)
- Dawn wire behavior tests: [src/dawn/tests/unittests/wire](https://github.com/google/dawn/tree/main/src/dawn/tests/unittests/wire)

## License

BSD 3-Clause. See `LICENSE`.
