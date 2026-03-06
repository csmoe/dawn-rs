import argparse
import os
import shutil
import subprocess
import sys
from pathlib import Path


def repo_root() -> Path:
    return Path(__file__).resolve().parent.parent


def run(
    cmd: list[str], cwd: Path | None = None, env: dict[str, str] | None = None
) -> None:
    subprocess.check_call(cmd, cwd=str(cwd) if cwd else None, env=env)


def ensure_depot_tools(root: Path) -> Path:
    depot = root / "third_party" / "depot_tools"
    has_tools = (depot / "gn").exists() or (depot / "gn.bat").exists()
    if has_tools:
        return depot

    if depot.exists():
        shutil.rmtree(depot)
    depot.parent.mkdir(parents=True, exist_ok=True)

    urls = [
        "https://chromium.googlesource.com/chromium/tools/depot_tools.git",
    ]
    last_err: Exception | None = None
    for url in urls:
        try:
            run(["git", "clone", "--depth", "10", url, str(depot)])
            return depot
        except Exception as e:  # best-effort fallback to next mirror
            last_err = e
    raise RuntimeError(f"Failed to fetch depot_tools into {depot}: {last_err}")


def initialize_depot_tools(depot: Path) -> None:
    marker = depot / "python3_bin_reldir.txt"
    if marker.exists():
        return

    env = os.environ.copy()
    env["PATH"] = str(depot) + os.pathsep + env.get("PATH", "")

    if os.name != "nt":
        # First try a lightweight bootstrap of CIPD/python wrappers.
        bootstrap = depot / "ensure_bootstrap"
        if bootstrap.exists():
            try:
                run([str(bootstrap)], cwd=depot, env=env)
            except subprocess.CalledProcessError:
                pass
            if marker.exists():
                return

    # Fallback to full depot_tools updater/initializer.
    updater = depot / (
        "update_depot_tools.bat" if os.name == "nt" else "update_depot_tools"
    )
    if updater.exists():
        env2 = env.copy()
        env2.pop("DEPOT_TOOLS_UPDATE", None)
        run([str(updater)], cwd=depot, env=env2)
    if not marker.exists():
        raise RuntimeError(
            f"depot_tools bootstrap incomplete: missing {marker}. "
            "Run gclient/update_depot_tools/ensure_bootstrap manually."
        )


def tool_in_depot_tools(root: Path, name: str) -> str:
    depot = root / "third_party" / "depot_tools"
    candidates = [depot / name]
    if os.name == "nt":
        candidates = [depot / f"{name}.bat", depot / f"{name}.exe", depot / name]
    for c in candidates:
        if c.exists():
            return str(c)
    found = shutil.which(name)
    if found:
        return found
    raise RuntimeError(f"Failed to locate `{name}` in depot_tools or PATH")


def build_dawn(tag: str, cache_root: Path, runner_os: str) -> Path:
    script = repo_root() / "scripts" / "download_dawn_source.py"
    dawn_root = subprocess.check_output(
        [sys.executable, str(script), "--tag", tag, "--output", str(cache_root)],
        text=True,
    ).strip()
    root = Path(dawn_root)
    if not (root / "BUILD.gn").is_file():
        raise RuntimeError(f"Invalid Dawn source root: {root}")

    depot_tools = ensure_depot_tools(root)
    initialize_depot_tools(depot_tools)
    env = os.environ.copy()
    env.setdefault("DEPOT_TOOLS_UPDATE", "0")
    env.setdefault("GCLIENT_PY3", "1")
    env["PATH"] = str(depot_tools) + os.pathsep + env.get("PATH", "")
    env.setdefault("DEPOT_TOOLS_WIN_TOOLCHAIN", "0")

    gn = tool_in_depot_tools(root, "gn")
    gclient = tool_in_depot_tools(root, "gclient")
    autoninja = tool_in_depot_tools(root, "autoninja")
    out_dir = root / "out/Release"

    deps_marker = root / ".deps_fetched"
    gclient_cfg = root / ".gclient"
    standalone_cfg = root / "scripts" / "standalone.gclient"
    if not deps_marker.exists():
        if not standalone_cfg.is_file():
            raise RuntimeError(f"Missing standalone.gclient template: {standalone_cfg}")
        shutil.copyfile(standalone_cfg, gclient_cfg)
        run([gclient, "sync"], cwd=root, env=env)
        deps_marker.touch()

    gn_args = [
        "is_debug=false",
        "is_component_build=false",
        "dawn_build_samples=false",
        "dawn_build_tests=false",
        "dawn_build_benchmarks=false",
        "tint_build_tests=false",
        "tint_build_ir_binary=false",
    ]
    if runner_os == "Windows":
        gn_args.extend(
            [
                'target_cpu="x64"',
            ]
        )
    run([gn, "gen", str(out_dir), f"--args={' '.join(gn_args)}"], cwd=root, env=env)

    target_build = [
        autoninja,
        "-C",
        str(out_dir),
        "webgpu_dawn",
        "dawn_proc",
        "dawn_wire",
    ]
    try:
        run(target_build, cwd=root, env=env)
    except subprocess.CalledProcessError:
        print("Targeted Dawn build failed, falling back to full build.", flush=True)
        run([autoninja, "-C", str(out_dir)], cwd=root, env=env)
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
