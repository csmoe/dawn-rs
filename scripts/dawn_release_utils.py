import json
import os
import platform
import shutil
import tarfile
import urllib.request
import zipfile
from pathlib import Path


def github_headers() -> dict:
    headers = {
        "User-Agent": "dawn-ci",
        "Accept": "application/vnd.github+json",
    }
    token = os.environ.get("GITHUB_TOKEN", "").strip()
    if token:
        headers["Authorization"] = f"Bearer {token}"
    return headers


def http_get_json(url: str) -> dict:
    req = urllib.request.Request(url, headers=github_headers())
    with urllib.request.urlopen(req) as resp:
        return json.load(resp)


def download_file(url: str, dest_path: str) -> None:
    req = urllib.request.Request(url, headers=github_headers())
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
    return root


def detect_platform_hint() -> str:
    sys_name = platform.system().lower()
    if "darwin" in sys_name:
        return "macos"
    if "windows" in sys_name:
        return "windows"
    return "linux"


def score_asset(name: str, platform_hint: str) -> int:
    n = name.lower()
    score = 0
    os_markers = {
        "linux": ("linux", "ubuntu"),
        "macos": ("mac", "macos", "osx", "darwin"),
        "windows": ("windows", "win", "msvc"),
    }

    target_markers = os_markers.get(platform_hint, ())
    if any(marker in n for marker in target_markers):
        score += 4

    if platform_hint == "linux":
        if "x86_64" in n or "amd64" in n:
            score += 2
        if "aarch64" in n or "arm64" in n:
            score += 1
    elif platform_hint == "macos":
        if "arm64" in n or "aarch64" in n:
            score += 2
        if "x86_64" in n or "amd64" in n:
            score += 1
    elif platform_hint == "windows":
        if "x86_64" in n or "amd64" in n:
            score += 2

    for os_name, markers in os_markers.items():
        if os_name == platform_hint:
            continue
        if any(marker in n for marker in markers):
            score -= 3
            break

    if n.endswith(".tar.gz") or n.endswith(".tgz"):
        score += 1
    if n.endswith(".zip"):
        score += 1
    return score


def release_latest() -> dict:
    return http_get_json("https://api.github.com/repos/google/dawn/releases/latest")


def release_by_tag(tag: str) -> dict:
    return http_get_json(
        f"https://api.github.com/repos/google/dawn/releases/tags/{tag}"
    )


def default_tag_from_repo_root(repo_root: Path) -> str:
    version_file = repo_root / "DAWN_VERSION"
    if not version_file.exists():
        return ""
    return version_file.read_text(encoding="utf-8").strip()


def select_prebuilt_asset(release_json: dict, platform_hint: str) -> dict:
    assets = release_json.get("assets", [])
    assets = sorted(
        assets,
        key=lambda a: score_asset(a.get("name", ""), platform_hint),
        reverse=True,
    )
    for asset in assets:
        if score_asset(asset.get("name", ""), platform_hint) > 0:
            return asset

    release_tag = release_json.get("tag_name", "<unknown>")
    debug = "\n".join(
        f"score={score_asset(a.get('name', ''), platform_hint):>2} {a.get('name', '')}"
        for a in assets
    )
    raise RuntimeError(
        f"Failed to find prebuilt asset for platform={platform_hint} in release {release_tag}\n{debug}"
    )


def extract_prebuilt_asset(asset: dict, output: str) -> str:
    url = asset.get("browser_download_url", "")
    name = asset.get("name", "")
    if not url or not name:
        raise RuntimeError("Selected release asset is missing name/url")

    output = os.path.abspath(output)
    if os.path.exists(output):
        shutil.rmtree(output)
    os.makedirs(output, exist_ok=True)
    archive = os.path.join(output, name)
    download_file(url, archive)
    extract_archive(archive, output)
    os.remove(archive)
    return find_first_dir(output)
