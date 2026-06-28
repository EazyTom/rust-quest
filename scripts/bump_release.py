#!/usr/bin/env python3
"""Bump rust-quest semver, update README, and draft What's New from git history.

Used by .github/workflows/bump-version.yml — no cargo build or release artifacts.
"""

from __future__ import annotations

import argparse
import re
import subprocess
import sys
from collections import defaultdict
from datetime import date
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
CARGO_TOML = ROOT / "Cargo.toml"
CARGO_LOCK = ROOT / "Cargo.lock"
README = ROOT / "README.md"

VERSION_RE = re.compile(r'^version\s*=\s*"(\d+\.\d+\.\d+)"\s*$', re.MULTILINE)
TAG_RE = re.compile(r"^v(\d+\.\d+\.\d+)$")

# Paths that count as player-facing / gameplay (included in What's New).
GAMEPLAY_PREFIXES = (
    "src/game/hub.rs",
    "src/game/quiz.rs",
    "src/game/epic.rs",
    "src/game/narrative.rs",
    "src/game/achievements.rs",
    "src/game/audio.rs",
    "src/game/ui/map.rs",
    "src/game/ui/input.rs",
    "src/game/ui/copy.rs",
)
PROGRESS_PREFIXES = (
    "src/game/state.rs",
    "src/game/progress.rs",
    "src/game/xp.rs",
)
QUEST_PREFIX = "src/topics/"
RESOURCE_PREFIX = "src/resources/"
MUSIC_PREFIX = "assets/music/"

# Commits touching only these (plus docs/meta) are omitted from What's New.
COSMETIC_PREFIXES = (
    "src/game/ui/retro.rs",
)
META_PREFIXES = (
    "README.md",
    "CONTRIBUTING.md",
    "AGENTS.md",
    ".github/",
    "scripts/bump_release.py",
    "scripts/run_tests",
    "Cargo.lock",
)


def run(*args: str, check: bool = True) -> str:
    result = subprocess.run(
        args,
        cwd=ROOT,
        text=True,
        capture_output=True,
        check=False,
    )
    if check and result.returncode != 0:
        print(result.stderr or result.stdout, file=sys.stderr)
        sys.exit(result.returncode)
    return (result.stdout or "").strip()


def read_version() -> str:
    text = CARGO_TOML.read_text(encoding="utf-8")
    match = VERSION_RE.search(text)
    if not match:
        sys.exit("Could not parse version from Cargo.toml")
    return match.group(1)


def bump_version(current: str, kind: str) -> str:
    major, minor, patch = (int(x) for x in current.split("."))
    if kind == "major":
        return f"{major + 1}.0.0"
    if kind == "minor":
        return f"{major}.{minor + 1}.0"
    if kind == "patch":
        return f"{major}.{minor}.{patch + 1}"
    sys.exit(f"Unknown bump kind: {kind}")


def write_cargo_toml(new_version: str) -> None:
    text = CARGO_TOML.read_text(encoding="utf-8")
    updated, n = VERSION_RE.subn(f'version = "{new_version}"', text, count=1)
    if n != 1:
        sys.exit("Failed to update Cargo.toml version")
    CARGO_TOML.write_text(updated, encoding="utf-8")


def write_cargo_lock(old_version: str, new_version: str) -> None:
    if not CARGO_LOCK.is_file():
        return
    text = CARGO_LOCK.read_text(encoding="utf-8")
    block = f'name = "rust-quest"\nversion = "{old_version}"'
    replacement = f'name = "rust-quest"\nversion = "{new_version}"'
    if block not in text:
        sys.exit("Could not find rust-quest version block in Cargo.lock")
    CARGO_LOCK.write_text(text.replace(block, replacement, 1), encoding="utf-8")


def latest_release_tag() -> str | None:
    tags = run("git", "tag", "--list", "v*.*.*", "--sort=-v:refname", check=False)
    for line in tags.splitlines():
        line = line.strip()
        if TAG_RE.match(line):
            return line
    return None


def normalize_subject(subject: str) -> str:
    subject = subject.strip()
    for prefix in (
        "feat:",
        "fix:",
        "chore:",
        "docs:",
        "refactor:",
        "test:",
        "ci:",
    ):
        if subject.lower().startswith(prefix):
            return subject[len(prefix) :].strip()
    return subject


def classify_files(files: list[str]) -> list[str]:
    """Return category labels for a commit's changed files."""
    categories: set[str] = set()
    for f in files:
        f = f.replace("\\", "/")
        if f.startswith(QUEST_PREFIX):
            categories.add("Quests & learning")
        elif any(f.startswith(p) for p in PROGRESS_PREFIXES):
            categories.add("Progress & save")
        elif any(f.startswith(p) for p in GAMEPLAY_PREFIXES):
            categories.add("Gameplay & systems")
        elif f.startswith(RESOURCE_PREFIX):
            categories.add("Resources & links")
        elif f.startswith(MUSIC_PREFIX):
            categories.add("Audio")
        elif any(f.startswith(p) for p in COSMETIC_PREFIXES):
            categories.add("_cosmetic")
        elif any(f.startswith(p) or f == p.rstrip("/") for p in META_PREFIXES):
            categories.add("_meta")

    visible = [c for c in sorted(categories) if not c.startswith("_")]
    if visible:
        return visible

    if categories == {"_cosmetic"} or categories <= {"_cosmetic", "_meta"}:
        return []

    if categories == {"_meta"}:
        return []

    return ["Other"]


def commits_since_tag(since_tag: str | None) -> list[tuple[str, list[str]]]:
    log_range = f"{since_tag}..HEAD" if since_tag else "HEAD"
    raw = run(
        "git",
        "log",
        log_range,
        "--no-merges",
        "--pretty=format:%H%x09%s",
        "--name-only",
        check=False,
    )
    if not raw:
        return []

    commits: list[tuple[str, list[str]]] = []
    current_subject: str | None = None
    current_files: list[str] = []

    for line in raw.splitlines():
        if "\t" in line:
            if current_subject is not None:
                commits.append((current_subject, current_files))
            hash_part, subject = line.split("\t", 1)
            _ = hash_part
            current_subject = subject
            current_files = []
        elif line.strip():
            current_files.append(line.strip())

    if current_subject is not None:
        commits.append((current_subject, current_files))

    return commits


def build_whats_new_body(
    since_tag: str | None, new_version: str, release_date: str
) -> str:
    grouped: dict[str, list[str]] = defaultdict(list)
    seen: set[str] = set()

    for subject, files in commits_since_tag(since_tag):
        categories = classify_files(files)
        if not categories:
            continue
        bullet = normalize_subject(subject)
        if not bullet or bullet in seen:
            continue
        seen.add(bullet)
        primary = categories[0]
        grouped[primary].append(f"- {bullet}")

    if not grouped:
        grouped["Gameplay & systems"].append(
            "- Maintenance and internal improvements since last release"
        )

    order = [
        "Gameplay & systems",
        "Progress & save",
        "Quests & learning",
        "Resources & links",
        "Audio",
        "Other",
    ]

    lines = [f"### v{new_version} — {release_date}", ""]
    for key in order:
        items = grouped.get(key)
        if not items:
            continue
        lines.append(f"**{key}**")
        lines.extend(items)
        lines.append("")

    return "\n".join(lines).rstrip() + "\n"


def update_readme(new_version: str, whats_new_entry: str) -> None:
    text = README.read_text(encoding="utf-8")

    text = re.sub(
        r"^\*\*Version \d+\.\d+\.\d+\*\*\s*$",
        f"**Version {new_version}**",
        text,
        count=1,
        flags=re.MULTILINE,
    )
    text = re.sub(
        r"^## Game Features \(v\d+\.\d+\.\d+\)\s*$",
        f"## Game Features (v{new_version})",
        text,
        count=1,
        flags=re.MULTILINE,
    )
    text = re.sub(
        r"(← bump \[package\]\.version for releases \(currently )\d+\.\d+\.\d+(\))",
        rf"\g<1>{new_version}\2",
        text,
        count=1,
    )

    version_note = (
        "> **Version:** `[package].version` in [`Cargo.toml`](Cargo.toml) is the source of truth "
        f"(currently **{new_version}**). The hub reads it via [`src/version.rs`](src/version.rs). "
        "Releases are bumped by the [**Bump version**](../../actions/workflows/bump-version.yml) "
        "GitHub Action — no binaries are published; clone and `cargo run`."
    )
    text = re.sub(
        r"> \*\*Version:\*\*[^\n]+\n",
        version_note + "\n",
        text,
        count=1,
    )

    anchor = "<!-- bump-release:whats-new -->"
    if anchor not in text:
        insert = (
            "\n---\n\n"
            "## What's New\n\n"
            f"{anchor}\n\n"
            f"### v{new_version} — initial catalog entry\n\n"
            "See [Game Features](#game-features-v"
            + new_version.replace(".", "")
            + ") for the full v"
            + new_version
            + " feature list.\n"
        )
        marker = "\n---\n\n## Game Features"
        if marker not in text:
            sys.exit("README missing Game Features section")
        text = text.replace(marker, insert + marker, 1)
    else:
        text = text.replace(
            anchor,
            f"{anchor}\n\n{whats_new_entry}",
            1,
        )

    README.write_text(text, encoding="utf-8")


def main() -> None:
    parser = argparse.ArgumentParser(description="Bump rust-quest release version")
    parser.add_argument(
        "--bump",
        choices=("patch", "minor", "major"),
        default="patch",
        help="Semver increment (default: patch)",
    )
    parser.add_argument(
        "--write",
        action="store_true",
        help="Write changes to Cargo.toml, Cargo.lock, and README.md",
    )
    parser.add_argument(
        "--print-version",
        action="store_true",
        help="Print the new version and exit (after --write)",
    )
    args = parser.parse_args()

    old_version = read_version()
    new_version = bump_version(old_version, args.bump)
    since_tag = latest_release_tag()
    release_date = date.today().isoformat()
    whats_new = build_whats_new_body(since_tag, new_version, release_date)

    print(f"Previous tag: {since_tag or '(none)'}")
    print(f"Version: {old_version} -> {new_version}")
    print()
    print("What's New preview:")
    print(whats_new)

    if not args.write:
        print("(dry run — pass --write to apply)")
        return

    write_cargo_toml(new_version)
    write_cargo_lock(old_version, new_version)
    update_readme(new_version, whats_new)

    if args.print_version:
        print(new_version)


if __name__ == "__main__":
    main()
