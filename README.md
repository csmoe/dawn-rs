# dawn-rs

Rust bindings and thin wrappers for Dawn WebGPU.

## Status

Active development. **Not approved for production use.**

## Prerequisites

1. Rust toolchain (`cargo` + stable toolchain).
2. A built Dawn tree.
3. `DAWN_ROOT` pointing to either:
   - the Dawn source root with build output directory that contains static libs (for example `dawn/out/Release`).

Build Dawn first by following the official guide:
- [Dawn build instructions](https://dawn.googlesource.com/dawn/+/HEAD/docs/building.md)

Windows note:
- Rust defaults to static CRT in many setups. If needed, build Dawn with `/MT`:
  - `cmake -GNinja -B out/Release -DCMAKE_BUILD_TYPE=Release -DCMAKE_MSVC_RUNTIME_LIBRARY=MultiThreaded -DCMAKE_POLICY_DEFAULT_CMP0091=NEW -DABSL_MSVC_STATIC_RUNTIME=ON`
- If you build Dawn with GN instead of CMake, use a static non-component build as well:
  - `gn gen out/Release --args="is_debug=false is_component_build=false is_clang=true target_cpu=\"x64\""`
  - `ninja -C out/Release`

## Quick Start

Run the native adapter example:

```bash
DAWN_ROOT=/path/to/dawn cargo run --example adapter-info
```

Run the native triangle example:

```bash
DAWN_ROOT=/path/to/dawn cargo run --example triangle
```

Run wire triangle example (`dawn-rs`):

```bash
DAWN_ROOT=/path/to/dawn cargo run --example triangle-wire --features wire
```

Run wire nya-cat example (`dawn-wgpu`):

```bash
DAWN_ROOT=/path/to/dawn cargo run -p dawn-wgpu --example nya-cat-wire --features wire
```

## Code Generation

Inputs:
- `--dawn-json`: path to `src/dawn/dawn.json` from Dawn source.
- `--api-header`: path to `include/webgpu/webgpu.h` from Dawn build/prebuilt artifacts.

Windows:

```bash
cargo run -p dawn-codegen --bin dawn_codegen -- \
  --dawn-json <path_to_dawn_json> \
  --api-header <path_to_webgpu_h> \
  --out-dir src/generated
```

macOS:

```bash
cargo run -p dawn-codegen --bin dawn_codegen -- \
  --dawn-json <path_to_dawn_json> \
  --api-header <path_to_webgpu_h> \
  --out-dir src/generated \
  --clang-arg --sysroot="$(xcrun --show-sdk-path --sdk macosx)"
```

Notes:
- Generated bindings are selected per OS/arch in `src/generated/mod.rs`.
- `build.rs` handles link directives and builds the C++ wire shim.
- Codegen is explicit; it does not run automatically during normal `cargo build`.

## Update Dawn API Snapshot

Use the helper script to fetch the latest Dawn release and regenerate bindings:

```bash
python3 scripts/update_dawn_release.py
```

Optional:
- `--force` to regenerate even if `DAWN_VERSION` is already latest.
- `DAWN_TAGS` env var to override generator tags (default: `dawn,native`).

## Upstream References

- Dawn API schema: [dawn.json](https://dawn.googlesource.com/dawn/+/HEAD/src/dawn/dawn.json)
- Generator notes: [docs/dawn/codegen.md](https://dawn.googlesource.com/dawn/+/HEAD/docs/dawn/codegen.md)
- C++ wrapper template: [generator/templates/api_cpp.h](https://dawn.googlesource.com/dawn/+/HEAD/generator/templates/api_cpp.h)

## License

BSD 3-Clause "New" or "Revised" License. See [LICENSE](LICENSE).
