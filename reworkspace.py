#!/usr/bin/python3

# How to use this tool:
# 1. cd to the root of this repo
# 2. python3 reworkspace.py
# 3. inspect Cargo.toml.new
# 4. mv -f Cargo.toml.new Cargo.toml
# 5. git commit -a

import sys
from pathlib import Path

root = Path("mdbook/src")
repos = []
for d0 in (root, root / "appendix"):
    for d in d0.iterdir():
        if (d / "src").is_dir() or (d / "examples").is_dir():
            repos.append(d)

def next_line(lines):
    try:
        return next(lines)
    except StopIteration:
        return None

with open("Cargo.toml", "r") as c:
    lines = iter(c.read().splitlines())
    with open("Cargo.toml.new", "w") as nc:
        while True:
            line = next_line(lines)
            if line is None:
                break
            if line == "members = [":
                print(line, file=nc)
                while True:
                    line = next_line(lines)
                    if line is None:
                        print("unclosed members directive", file=sys.stderr)
                        break
                    if line == "]":
                        break
                for p in sorted(repos):
                    print(f'  "{str(p)}",', file=nc)
                print("]", file=nc)
                continue
            print(line, file=nc)
