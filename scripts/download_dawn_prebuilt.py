import argparse
from pathlib import Path

from dawn_release_utils import (
    default_tag_from_repo_root,
    extract_prebuilt_asset,
    release_by_tag,
    select_prebuilt_asset,
)


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Download Dawn prebuilt archive by release tag"
    )
    parser.add_argument(
        "--platform",
        required=True,
        choices=["linux", "macos", "windows"],
        help="Target platform to select release asset",
    )
    parser.add_argument(
        "--output",
        required=True,
        help="Output directory to extract the artifact into",
    )
    parser.add_argument(
        "--tag",
        default="",
        help="Dawn release tag (defaults to DAWN_VERSION in repo root)",
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

    release_json = release_by_tag(tag)
    tag = release_json.get("tag_name", "")
    assets = release_json.get("assets", [])
    if not tag or not assets:
        raise RuntimeError(
            f"Failed to query google/dawn release for tag {args.tag or '<DAWN_VERSION>'}"
        )

    selected = select_prebuilt_asset(release_json, args.platform)
    dawn_root = extract_prebuilt_asset(selected, args.output)
    print(dawn_root)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
