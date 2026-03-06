import argparse
import os
import shutil
import subprocess
from pathlib import Path

from dawn_release_utils import default_tag_from_repo_root


def main() -> int:
    parser = argparse.ArgumentParser(description="Clone Dawn source at a specific tag")
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
    if os.path.isfile(os.path.join(dawn_root, "BUILD.gn")):
        print(dawn_root)
        return 0

    os.makedirs(output, exist_ok=True)
    if os.path.exists(dawn_root):
        shutil.rmtree(dawn_root)

    clone_urls = [
        # "https://dawn.googlesource.com/dawn",
        "https://github.com/google/dawn.git",
    ]
    last_err = None
    for url in clone_urls:
        try:
            subprocess.check_call(
                [
                    "git",
                    "clone",
                    "--depth",
                    "1",
                    "--branch",
                    tag,
                    url,
                    dawn_root,
                ]
            )
            last_err = None
            break
        except subprocess.CalledProcessError as e:
            last_err = e

    if last_err is not None:
        raise RuntimeError(f"Failed to clone Dawn tag {tag}: {last_err}")

    if not os.path.isfile(os.path.join(dawn_root, "BUILD.gn")):
        raise RuntimeError(f"Cloned Dawn tree is invalid: {dawn_root}")

    print(dawn_root)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
