#!/usr/bin/env python3
"""Update catalog.json with real SHA256 hashes and download_bytes
for all built .serein-extension plugin packages."""
import hashlib
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent
CATALOG = ROOT / "catalog.json"
PACKAGES = ROOT / "plugins" / "packages"

# Map package filename -> catalog entry id
PLUGIN_MAP = {
    "message-logger.serein-extension": "message-logger",
    "status-logger.serein-extension": "status-logger",
    "relation-logger.serein-extension": "relation-logger",
    "anti-ghost.serein-extension": "anti-ghost",
    "auto-reply.serein-extension": "auto-reply",
    "bulk-delete.serein-extension": "bulk-delete",
}

def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(65536), b""):
            h.update(chunk)
    return h.hexdigest()

def main():
    if not CATALOG.exists():
        print(f"ERROR: {CATALOG} not found")
        sys.exit(1)

    with open(CATALOG, "r", encoding="utf-8") as f:
        catalog = json.load(f)

    updated = 0
    for entry in catalog["entries"]:
        plugin_id = entry["manifest"]["id"]
        # Find the package file for this plugin id
        pkg_file = None
        for fname, pid in PLUGIN_MAP.items():
            if pid == plugin_id:
                pkg_file = PACKAGES / fname
                break
        if pkg_file is None or not pkg_file.exists():
            print(f"WARNING: No package file found for plugin '{plugin_id}', skipping")
            continue

        sha256 = sha256_file(pkg_file)
        size = pkg_file.stat().st_size

        entry["sha256"] = sha256
        entry["download_bytes"] = size
        print(f"Updated {plugin_id}: {size} bytes, sha256={sha256}")
        updated += 1

    with open(CATALOG, "w", encoding="utf-8") as f:
        json.dump(catalog, f, indent=2)
        f.write("\n")

    print(f"\nUpdated {updated} plugin entries in catalog.json")

if __name__ == "__main__":
    main()
