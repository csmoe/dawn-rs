import argparse
import os
from pathlib import Path

from dawn_release_utils import (
    default_tag_from_repo_root,
    download_file,
    extract_archive,
    release_by_tag,
)


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Download Dawn source tarball from release"
    )
    parser.add_argument(
        "--tag",
        default="",
        help="Dawn release tag (defaults to DAWN_VERSION in repo root)",
    )
    parser.add_argument(
        "--output",
        required=True,
        help="Directory where Dawn source will be extracted",
    )
    args = parser.parse_args()

    tag = args.tag.strip()
    if not tag:
        repo_root = Path(__file__).resolve().parent.parent
        tag = default_tag_from_repo_root(repo_root)
    if not tag:
        raise RuntimeError(
            "Missing release tag: pass --tag or provide DAWN_VERSION in repo root"
        )

    output = os.path.abspath(args.output)
    normalized_tag = tag[1:] if tag.startswith("v") else tag
    dawn_root = os.path.join(output, f"dawn-{normalized_tag}")
    if os.path.isfile(os.path.join(dawn_root, "CMakeLists.txt")):
        print(dawn_root)
        return 0

    release_json = release_by_tag(tag)
    tarball_url = release_json.get("tarball_url", "").strip()
    if not tarball_url:
        raise RuntimeError(f"Missing tarball_url in release payload for tag {tag}")

    os.makedirs(output, exist_ok=True)
    before_dirs = {
        entry
        for entry in os.listdir(output)
        if os.path.isdir(os.path.join(output, entry))
    }
    archive = os.path.join(output, "dawn.tar.gz")
    if os.path.exists(archive):
        os.remove(archive)
    download_file(tarball_url, archive)
    extract_archive(archive, output)
    os.remove(archive)

    if os.path.isfile(os.path.join(dawn_root, "CMakeLists.txt")):
        print(dawn_root)
        return 0

    after_dirs = {
        entry
        for entry in os.listdir(output)
        if os.path.isdir(os.path.join(output, entry))
    }
    created_dirs = [os.path.join(output, d) for d in sorted(after_dirs - before_dirs)]

    extracted = ""
    if len(created_dirs) == 1:
        extracted = created_dirs[0]
    else:
        for path in created_dirs:
            if os.path.isfile(os.path.join(path, "CMakeLists.txt")):
                extracted = path
                break
        if not extracted:
            for entry in sorted(after_dirs):
                path = os.path.join(output, entry)
                if os.path.isfile(os.path.join(path, "CMakeLists.txt")):
                    extracted = path
                    break
    if not extracted:
        raise RuntimeError(f"Failed to locate extracted Dawn source under {output}")

    if os.path.abspath(extracted) != os.path.abspath(dawn_root):
        if os.path.exists(dawn_root):
            raise RuntimeError(f"Destination already exists: {dawn_root}")
        os.rename(extracted, dawn_root)

    print(dawn_root)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
