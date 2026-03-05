import argparse
import os
import subprocess
import sys
from pathlib import Path


def repo_root() -> Path:
    return Path(__file__).resolve().parent.parent


def run(cmd: list[str], cwd: Path | None = None) -> None:
    subprocess.check_call(cmd, cwd=str(cwd) if cwd else None)


def build_dawn(tag: str, cache_root: Path, runner_os: str) -> Path:
    script = repo_root() / "scripts" / "download_dawn_source.py"
    dawn_root = subprocess.check_output(
        [sys.executable, str(script), "--tag", tag, "--output", str(cache_root)],
        text=True,
    ).strip()
    root = Path(dawn_root)
    if not (root / "CMakeLists.txt").is_file():
        raise RuntimeError(f"Invalid Dawn source root: {root}")

    deps_marker = root / ".deps_fetched"
    if not deps_marker.exists():
        run([sys.executable, "tools/fetch_dawn_dependencies.py"], cwd=root)
        deps_marker.touch()

    cmake_args = [
        "cmake",
        "-GNinja",
        "-S",
        str(root),
        "-B",
        str(root / "out/Debug"),
        "-DCMAKE_BUILD_TYPE=Debug",
        "-DBUILD_SHARED_LIBS=OFF",
        "-DDAWN_BUILD_SAMPLES=OFF",
        "-DDAWN_BUILD_TESTS=OFF",
        "-DTINT_BUILD_TESTS=OFF",
        "-DTINT_BUILD_IR_BINARY=OFF",
        "-DDAWN_BUILD_BENCHMARKS=OFF",
    ]
    if runner_os == "Windows":
        cmake_args.extend(
            [
                "-DCMAKE_MSVC_RUNTIME_LIBRARY=MultiThreadedDebug",
                "-DCMAKE_POLICY_DEFAULT_CMP0091=NEW",
                "-DABSL_MSVC_STATIC_RUNTIME=ON",
                "-Dprotobuf_MSVC_STATIC_RUNTIME=ON",
            ]
        )

    run(cmake_args)

    target_build = [
        "cmake",
        "--build",
        str(root / "out/Debug"),
        "--config",
        "Debug",
        "--parallel",
        "--target",
        "webgpu_dawn",
        "dawn_proc",
        "dawn_wire",
    ]
    run(target_build)
    return root


def main() -> int:
    parser = argparse.ArgumentParser(description="Download and build Dawn for CI")
    parser.add_argument("--tag", required=True, help="Dawn release tag")
    parser.add_argument(
        "--cache-root",
        required=True,
        help="Cache directory where Dawn source/build trees are stored",
    )
    parser.add_argument(
        "--runner-os",
        default=os.environ.get("RUNNER_OS", ""),
        help="GitHub runner OS name (Linux|Windows|macOS)",
    )
    args = parser.parse_args()

    runner_os = args.runner_os.strip()
    if not runner_os:
        raise RuntimeError("Missing runner OS; pass --runner-os or set RUNNER_OS")

    root = build_dawn(args.tag.strip(), Path(args.cache_root).resolve(), runner_os)
    github_env = os.environ.get("GITHUB_ENV", "").strip()
    if github_env:
        with open(github_env, "a", encoding="utf-8") as f:
            f.write(f"DAWN_ROOT={root}\n")
    print(f"Using DAWN tag={args.tag} DAWN_ROOT={root}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
