#!/usr/bin/env python3
import io
import json
import os
import platform
import shutil
import subprocess
import sys
import tarfile
import tempfile
import urllib.request
import zipfile


REPO_ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
VERSION_FILE = os.path.join(REPO_ROOT, "DAWN_VERSION")
OUT_DIR = os.path.join(REPO_ROOT, "src", "generated")


def read_text(path: str) -> str:
    try:
        with open(path, "r", encoding="utf-8") as f:
            return f.read().strip()
    except FileNotFoundError:
        return ""


def write_text(path: str, content: str) -> None:
    with open(path, "w", encoding="utf-8") as f:
        f.write(content)


def http_get_json(url: str) -> dict:
    req = urllib.request.Request(url, headers={"User-Agent": "dawn-codegen"})
    with urllib.request.urlopen(req) as resp:
        return json.load(resp)


def download_file(url: str, dest_path: str) -> None:
    req = urllib.request.Request(url, headers={"User-Agent": "dawn-codegen"})
    with urllib.request.urlopen(req) as resp:
        data = resp.read()
    with open(dest_path, "wb") as f:
        f.write(data)


def extract_archive(archive_path: str, dest_dir: str) -> None:
    if archive_path.endswith(".zip"):
        with zipfile.ZipFile(archive_path, "r") as zf:
            zf.extractall(dest_dir)
        return
    if archive_path.endswith(".tar.gz") or archive_path.endswith(".tgz"):
        with tarfile.open(archive_path, "r:gz") as tf:
            tf.extractall(dest_dir)
        return
    raise RuntimeError(f"Unsupported archive type: {archive_path}")


def find_first_dir(root: str) -> str:
    for entry in os.listdir(root):
        path = os.path.join(root, entry)
        if os.path.isdir(path):
            return path
    return ""


def find_webgpu_header(root: str) -> str:
    for dirpath, _, filenames in os.walk(root):
        for name in filenames:
            if name == "webgpu.h" and os.path.basename(dirpath) == "webgpu":
                return os.path.join(dirpath, name)
    return ""


def score_asset(name: str, platform_hint: str) -> int:
    n = name.lower()
    score = 0
    if platform_hint == "linux":
        if "linux" in n:
            score += 4
        if "x86_64" in n or "amd64" in n:
            score += 2
    elif platform_hint == "macos":
        if "mac" in n or "macos" in n or "osx" in n:
            score += 4
        if "arm64" in n or "aarch64" in n:
            score += 2
    elif platform_hint == "windows":
        if "windows" in n or "win" in n:
            score += 4
        if "x86_64" in n or "amd64" in n:
            score += 2
    if n.endswith(".tar.gz") or n.endswith(".tgz"):
        score += 1
    if n.endswith(".zip"):
        score += 1
    return score


def detect_platform_hint() -> str:
    sys_name = platform.system().lower()
    if "darwin" in sys_name:
        return "macos"
    if "windows" in sys_name:
        return "windows"
    return "linux"


def main() -> int:
    force = "--force" in sys.argv[1:]
    current_version = read_text(VERSION_FILE)
    release_json = http_get_json("https://api.github.com/repos/google/dawn/releases/latest")
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
    assets = release_json.get("assets", [])
    assets = sorted(
        assets,
        key=lambda a: score_asset(a.get("name", ""), platform_hint),
        reverse=True,
    )
    prebuilt_url = ""
    for asset in assets:
        if score_asset(asset.get("name", ""), platform_hint) > 0:
            prebuilt_url = asset.get("browser_download_url", "")
            break

    if not prebuilt_url:
        print("Missing prebuilt release asset; cannot locate headers.")
        return 1

    with tempfile.TemporaryDirectory() as tmp_dir:
        src_dir = os.path.join(tmp_dir, "src")
        prebuilt_dir = os.path.join(tmp_dir, "prebuilt")
        os.makedirs(src_dir, exist_ok=True)
        os.makedirs(prebuilt_dir, exist_ok=True)

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

        prebuilt_archive_name = os.path.basename(prebuilt_url)
        prebuilt_archive = os.path.join(tmp_dir, prebuilt_archive_name)
        download_file(prebuilt_url, prebuilt_archive)
        extract_archive(prebuilt_archive, prebuilt_dir)
        api_header = find_webgpu_header(prebuilt_dir)
        if not api_header:
            print("Failed to locate webgpu.h in prebuilt artifact.")
            return 1

        os.makedirs(OUT_DIR, exist_ok=True)
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
        subprocess.check_call(cmd, cwd=REPO_ROOT)

    write_text(VERSION_FILE, latest_tag + "\n")
    subprocess.check_call(["git", "status", "--porcelain"], cwd=REPO_ROOT)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
