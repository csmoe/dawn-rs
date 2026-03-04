#!/usr/bin/env python3
import os
import subprocess
import sys
import tempfile

from dawn_release_utils import (
    detect_platform_hint,
    download_file,
    extract_archive,
    extract_prebuilt_asset,
    find_first_dir,
    release_latest,
    select_prebuilt_asset,
)

REPO_ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
VERSION_FILE = os.path.join(REPO_ROOT, "DAWN_VERSION")
OUT_DIR = os.path.join(REPO_ROOT, "src", "generated")
DEFAULT_TAGS = "dawn,native"


def read_text(path: str) -> str:
    try:
        with open(path, "r", encoding="utf-8") as f:
            return f.read().strip()
    except FileNotFoundError:
        return ""


def write_text(path: str, content: str) -> None:
    with open(path, "w", encoding="utf-8") as f:
        f.write(content)


def find_webgpu_header(root: str) -> str:
    for dirpath, _, filenames in os.walk(root):
        for name in filenames:
            if name == "webgpu.h" and os.path.basename(dirpath) == "webgpu":
                return os.path.join(dirpath, name)
    return ""


def main() -> int:
    force = "--force" in sys.argv[1:]
    current_version = read_text(VERSION_FILE)
    release_json = release_latest()
    latest_tag = release_json.get("tag_name", "")
    if not latest_tag:
        print("No releases found for google/dawn; exiting.")
        return 0
    if latest_tag == current_version and not force:
        print(f"Dawn already at latest release {latest_tag}")
        return 0

    tarball_url = release_json.get("tarball_url", "")
    if not tarball_url:
        print("Missing tarball_url in release payload.")
        return 1

    platform_hint = detect_platform_hint()
    try:
        selected_asset = select_prebuilt_asset(release_json, platform_hint)
    except RuntimeError as e:
        print(str(e))
        return 1

    with tempfile.TemporaryDirectory() as tmp_dir:
        src_dir = os.path.join(tmp_dir, "src")
        prebuilt_dir = os.path.join(tmp_dir, "prebuilt")
        os.makedirs(src_dir, exist_ok=True)

        src_archive = os.path.join(tmp_dir, "dawn-src.tar.gz")
        download_file(tarball_url, src_archive)
        extract_archive(src_archive, src_dir)
        dawn_root = find_first_dir(src_dir)
        if not dawn_root:
            print("Failed to locate extracted dawn source root.")
            return 1
        dawn_json = os.path.join(dawn_root, "src", "dawn", "dawn.json")
        if not os.path.isfile(dawn_json):
            print(f"Missing dawn.json at {dawn_json}")
            return 1

        extract_prebuilt_asset(selected_asset, prebuilt_dir)
        api_header = find_webgpu_header(prebuilt_dir)
        if not api_header:
            print("Failed to locate webgpu.h in prebuilt artifact.")
            return 1

        os.makedirs(OUT_DIR, exist_ok=True)
        tags = os.environ.get("DAWN_TAGS", DEFAULT_TAGS).strip()
        cmd = [
            "cargo",
            "run",
            "-p",
            "dawn-codegen",
            "--bin",
            "dawn_codegen",
            "--",
            "--dawn-json",
            dawn_json,
            "--api-header",
            api_header,
            "--out-dir",
            OUT_DIR,
        ]
        if tags:
            cmd.extend(["--tags", tags])
        subprocess.check_call(cmd, cwd=REPO_ROOT)

    write_text(VERSION_FILE, latest_tag + "\n")
    subprocess.check_call(["git", "status", "--porcelain"], cwd=REPO_ROOT)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
